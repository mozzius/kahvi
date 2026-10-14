//! Compiles a parsed pattern into a program for the backtracking matcher.
//! Each construct follows its matcher in §22.2.2 (CompilePattern,
//! CompileSubpattern, CompileAtom, RepeatMatcher, ...); `back` is the
//! spec's _direction_ being ~backward~ (inside a lookbehind).

use crate::charset::{CharSet, Fold, Rer, canonicalize_ranges, word_characters};
use crate::parser::Node;

/// A matcher for one character.
#[derive(Clone, Debug)]
pub(crate) enum Item {
    /// Canonicalize(rer, ch) = c (c is already canonical).
    Char { c: u32, fold: Fold },
    /// CharacterSetMatcher: Canonicalize(rer, ch) is in the (canonicalized) set, unless inverted.
    Set {
        set: usize,
        invert: bool,
        fold: Fold,
    },
    /// `.`
    Any { dot_all: bool },
}

#[derive(Clone, Debug)]
pub(crate) enum Inst {
    One {
        item: Item,
        back: bool,
    },
    /// `^` and `$`.
    Start {
        multiline: bool,
    },
    End {
        multiline: bool,
    },
    /// `\b` / `\B`, with the WordCharacters(rer) set.
    WordBoundary {
        negate: bool,
        word: usize,
    },
    /// Try `x`; on failure, `y`.
    Split {
        x: usize,
        y: usize,
    },
    Jump(usize),
    /// Entering capturing group `n` (records where it started).
    GroupStart(usize),
    /// Leaving capturing group `n`: sets its capture.
    GroupEnd {
        group: usize,
        back: bool,
    },
    /// RepeatMatcher steps 3-4: clears the captures of groups first..first+count.
    ClearGroups {
        first: usize,
        count: usize,
    },
    BackRef {
        groups: Vec<usize>,
        fold: Fold,
        back: bool,
    },
    /// A lookaround whose body starts at the next instruction and ends with
    /// LookEnd; matching continues at `next`.
    Look {
        negate: bool,
        next: usize,
    },
    LookEnd,
    /// A quantified atom: sets the iteration counter `reg` to 0.
    RepeatStart {
        reg: usize,
    },
    /// Decides whether to run the body (at the next instruction) again or leave to `exit`.
    RepeatHead {
        reg: usize,
        min: u64,
        max: Option<u64>,
        greedy: bool,
        exit: usize,
    },
    /// The end of the body: the empty check, then back to `head`.
    RepeatEnd {
        reg: usize,
        min: u64,
        head: usize,
    },
    /// A quantified single-character atom (no captures inside).
    SimpleRepeat {
        item: Item,
        back: bool,
        min: u64,
        max: Option<u64>,
        greedy: bool,
    },
    Match,
}

pub(crate) struct Program {
    pub insts: Vec<Inst>,
    /// Canonicalized character sets.
    pub sets: Vec<Vec<(u32, u32)>>,
    /// [[RegExpRecord]].[[CapturingGroupsCount]] + 1 groups.
    pub groups: usize,
    pub registers: usize,
    /// HasEitherUnicodeFlag: the input is read as code points.
    pub unicode: bool,
    /// What a match must begin with, when that is known: one of these
    /// steps succeeds at the start position of every match. Lets the
    /// matcher reject a start position without running the program.
    pub firsts: Option<Vec<First>>,
}

/// A step every match takes before consuming anything else.
#[derive(Clone, Debug)]
pub(crate) enum First {
    /// One character matched forward by this item.
    Item(Item),
    /// `^` without the multiline flag: only position 0.
    InputStart,
}

/// The first steps of `insts` from instruction 0, following only the
/// instructions that neither consume input nor test it; `None` when some
/// path tests anything else first (lookarounds, `\b`, `$`, back
/// references, a loop body that can match empty, ...).
fn first_steps(insts: &[Inst]) -> Option<Vec<First>> {
    let mut firsts = Vec::new();
    let mut seen = vec![false; insts.len()];
    let mut todo = vec![0];
    while let Some(pc) = todo.pop() {
        if std::mem::replace(&mut seen[pc], true) {
            continue;
        }
        match &insts[pc] {
            Inst::One { item, back: false } => firsts.push(First::Item(item.clone())),
            Inst::Start { multiline: false } => firsts.push(First::InputStart),
            Inst::Split { x, y } => todo.extend([*x, *y]),
            Inst::Jump(t) => todo.push(*t),
            Inst::GroupStart(_)
            | Inst::GroupEnd { back: false, .. }
            | Inst::ClearGroups { .. }
            | Inst::RepeatStart { .. } => todo.push(pc + 1),
            // Reached through RepeatStart, so the iteration count is 0.
            Inst::RepeatHead { min, max, exit, .. } => {
                if *max != Some(0) {
                    todo.push(pc + 1);
                }
                if *min == 0 {
                    todo.push(*exit);
                }
            }
            Inst::SimpleRepeat {
                item,
                back: false,
                min,
                max,
                ..
            } => {
                if *max != Some(0) {
                    firsts.push(First::Item(item.clone()));
                }
                if *min == 0 {
                    todo.push(pc + 1);
                }
            }
            _ => return None,
        }
    }
    Some(firsts)
}

struct Compiler {
    insts: Vec<Inst>,
    sets: Vec<Vec<(u32, u32)>>,
    registers: usize,
}

/// CompilePattern(pattern, rer) (§22.2.2.2)
pub(crate) fn compile(node: &Node, rer: Rer, group_count: usize) -> Program {
    let mut c = Compiler {
        insts: Vec::new(),
        sets: Vec::new(),
        registers: 0,
    };
    // The whole match is group 0.
    c.insts.push(Inst::GroupStart(0));
    c.node(node, rer, false);
    c.insts.push(Inst::GroupEnd {
        group: 0,
        back: false,
    });
    c.insts.push(Inst::Match);
    Program {
        firsts: first_steps(&c.insts),
        insts: c.insts,
        sets: c.sets,
        groups: group_count + 1,
        registers: c.registers,
        unicode: rer.either_unicode(),
    }
}

impl Compiler {
    fn here(&self) -> usize {
        self.insts.len()
    }

    fn add_set(&mut self, ranges: Vec<(u32, u32)>) -> usize {
        self.sets.push(ranges);
        self.sets.len() - 1
    }

    /// A single-character matcher for an atom, if it is one.
    fn item(&mut self, node: &Node, rer: Rer) -> Option<Item> {
        let fold = Fold::of(rer);
        match node {
            Node::Char(c) => Some(Item::Char {
                c: fold.canonicalize(*c),
                fold,
            }),
            Node::Dot => Some(Item::Any {
                dot_all: rer.dot_all,
            }),
            Node::Class(set, invert) if set.strings.is_empty() => {
                let set = self.add_set(canonicalize_ranges(&set.ranges, &fold));
                Some(Item::Set {
                    set,
                    invert: *invert,
                    fold,
                })
            }
            _ => None,
        }
    }

    fn node(&mut self, node: &Node, rer: Rer, back: bool) {
        if let Some(item) = self.item(node, rer) {
            self.insts.push(Inst::One { item, back });
            return;
        }
        match node {
            Node::Empty => {}
            Node::Char(_) | Node::Dot => unreachable!("single-character atoms are items"),
            Node::Class(set, _) => self.class_with_strings(set, rer, back),
            Node::Start => self.insts.push(Inst::Start {
                multiline: rer.multiline,
            }),
            Node::End => self.insts.push(Inst::End {
                multiline: rer.multiline,
            }),
            Node::WordBoundary(negate) => {
                let word = self.add_set(word_characters(rer).ranges);
                self.insts.push(Inst::WordBoundary {
                    negate: *negate,
                    word,
                });
            }
            Node::Look {
                behind,
                negate,
                body,
            } => {
                let at = self.here();
                self.insts.push(Inst::Look {
                    negate: *negate,
                    next: 0,
                });
                // Lookahead matches forward, lookbehind backward, whatever the enclosing direction.
                self.node(body, rer, *behind);
                self.insts.push(Inst::LookEnd);
                let next = self.here();
                if let Inst::Look { next: n, .. } = &mut self.insts[at] {
                    *n = next;
                }
            }
            Node::Group(Some(index), body) => {
                self.insts.push(Inst::GroupStart(*index));
                self.node(body, rer, back);
                self.insts.push(Inst::GroupEnd {
                    group: *index,
                    back,
                });
            }
            Node::Group(None, body) => self.node(body, rer, back),
            Node::Modified(inner, body) => self.node(body, *inner, back),
            Node::BackRef(groups) => self.insts.push(Inst::BackRef {
                groups: groups.clone(),
                fold: Fold::of(rer),
                back,
            }),
            Node::NamedBackRef(_) => unreachable!("resolved by the parser"),
            Node::Alt(alternatives) => {
                self.alternatives(alternatives.iter().map(|a| (a, rer)).collect(), back)
            }
            Node::Seq(terms) => {
                // MatchSequence: with direction backward, the terms match right to left.
                if back {
                    for t in terms.iter().rev() {
                        self.node(t, rer, back);
                    }
                } else {
                    for t in terms {
                        self.node(t, rer, back);
                    }
                }
            }
            Node::Repeat {
                body,
                min,
                max,
                greedy,
                first_group,
                group_count,
            } => self.repeat(
                body,
                *min,
                *max,
                *greedy,
                *first_group,
                *group_count,
                rer,
                back,
            ),
        }
    }

    /// MatchTwoAlternatives, chained: try each alternative in order.
    fn alternatives(&mut self, alternatives: Vec<(&Node, Rer)>, back: bool) {
        let mut jumps = Vec::new();
        let last = alternatives.len().saturating_sub(1);
        for (i, (alt, rer)) in alternatives.into_iter().enumerate() {
            if i == last {
                self.node(alt, rer, back);
                break;
            }
            let split = self.here();
            self.insts.push(Inst::Split { x: split + 1, y: 0 });
            self.node(alt, rer, back);
            jumps.push(self.here());
            self.insts.push(Inst::Jump(0));
            let next = self.here();
            if let Inst::Split { y, .. } = &mut self.insts[split] {
                *y = next;
            }
        }
        let end = self.here();
        for j in jumps {
            self.insts[j] = Inst::Jump(end);
        }
    }

    /// CompileAtom for a CharacterClass whose CharSet contains strings
    /// (only with the `v` flag): the strings longest first, then the
    /// single characters, then the empty string.
    fn class_with_strings(&mut self, set: &CharSet, rer: Rer, back: bool) {
        let mut strings: Vec<&Vec<u32>> = set.strings.iter().filter(|s| !s.is_empty()).collect();
        strings.sort_by_key(|s| std::cmp::Reverse(s.len()));
        let seqs: Vec<Node> = strings
            .into_iter()
            .map(|s| Node::Seq(s.iter().map(|&c| Node::Char(c)).collect()))
            .collect();
        let mut alts: Vec<Node> = seqs;
        if !set.ranges.is_empty() {
            alts.push(Node::Class(CharSet::from_ranges(set.ranges.clone()), false));
        }
        if set.strings.contains(&Vec::new()) {
            alts.push(Node::Empty);
        }
        if alts.is_empty() {
            // An empty set never matches.
            alts.push(Node::Class(CharSet::empty(), false));
        }
        self.alternatives(alts.iter().map(|a| (a, rer)).collect(), back);
    }

    /// RepeatMatcher (§22.2.2.3.1)
    #[allow(clippy::too_many_arguments)]
    fn repeat(
        &mut self,
        body: &Node,
        min: u64,
        max: Option<u64>,
        greedy: bool,
        first_group: usize,
        group_count: usize,
        rer: Rer,
        back: bool,
    ) {
        // 1. If max = 0, return c(x).
        if max == Some(0) {
            return;
        }
        if let Some(item) = self.item(body, rer) {
            self.insts.push(Inst::SimpleRepeat {
                item,
                back,
                min,
                max,
                greedy,
            });
            return;
        }
        let reg = self.registers;
        self.registers += 1;
        self.insts.push(Inst::RepeatStart { reg });
        let head = self.here();
        self.insts.push(Inst::RepeatHead {
            reg,
            min,
            max,
            greedy,
            exit: 0,
        });
        if group_count > 0 {
            self.insts.push(Inst::ClearGroups {
                first: first_group,
                count: group_count,
            });
        }
        self.node(body, rer, back);
        self.insts.push(Inst::RepeatEnd { reg, min, head });
        let exit = self.here();
        if let Inst::RepeatHead { exit: e, .. } = &mut self.insts[head] {
            *e = exit;
        }
    }
}
