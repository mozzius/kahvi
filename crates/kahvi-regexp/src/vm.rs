//! The backtracking matcher. The spec defines matching with continuations
//! (§22.2.2); this runs the same search with an explicit stack of choice
//! points and undo records, so input length never turns into Rust stack depth.

use crate::charset::{Fold, is_line_terminator};
use crate::compiler::{First, Inst, Item, Program};

const NONE: usize = usize::MAX;

/// Undo records and choice points.
enum Bt {
    /// Resume at `pc` with `pos`.
    Branch {
        pc: usize,
        pos: usize,
    },
    /// Restore a capture slot.
    Capture {
        slot: usize,
        old: usize,
    },
    /// Restore a group's start register.
    GroupStart {
        group: usize,
        old: usize,
    },
    Counter {
        reg: usize,
        old: u64,
    },
    IterStart {
        reg: usize,
        old: usize,
    },
    /// A lazy loop's postponed choice to run the body once more.
    EnterBody {
        head: usize,
        pos: usize,
    },
    /// A greedy single-character loop that can give back characters.
    Greedy {
        pc: usize,
        start: usize,
        pos: usize,
        count: u64,
        min: u64,
    },
    /// A lazy single-character loop that can take one more character.
    Lazy {
        pc: usize,
        pos: usize,
        count: u64,
    },
}

/// A matcher's buffers, kept between matches.
#[derive(Default)]
pub(crate) struct Scratch {
    caps: Vec<usize>,
    group_starts: Vec<usize>,
    counters: Vec<u64>,
    iter_starts: Vec<usize>,
    stack: Vec<Bt>,
}

pub(crate) struct Matcher<'a> {
    prog: &'a Program,
    input: &'a [u16],
    /// Two slots per group: start and end (NONE when undefined).
    pub caps: Vec<usize>,
    group_starts: Vec<usize>,
    counters: Vec<u64>,
    iter_starts: Vec<usize>,
    stack: Vec<Bt>,
}

fn is_lead(u: u16) -> bool {
    (0xD800..=0xDBFF).contains(&u)
}

fn is_trail(u: u16) -> bool {
    (0xDC00..=0xDFFF).contains(&u)
}

impl<'a> Matcher<'a> {
    /// A matcher that reuses the buffers of an earlier one (matching runs
    /// once per start position, so allocating them each time dominates).
    pub fn with_scratch(prog: &'a Program, input: &'a [u16], scratch: Scratch) -> Matcher<'a> {
        let Scratch {
            mut caps,
            mut group_starts,
            mut counters,
            mut iter_starts,
            mut stack,
        } = scratch;
        caps.clear();
        caps.resize(prog.groups * 2, NONE);
        group_starts.clear();
        group_starts.resize(prog.groups, NONE);
        counters.clear();
        counters.resize(prog.registers, 0);
        iter_starts.clear();
        iter_starts.resize(prog.registers, NONE);
        stack.clear();
        Matcher {
            prog,
            input,
            caps,
            group_starts,
            counters,
            iter_starts,
            stack,
        }
    }

    /// The buffers, for the next `with_scratch`.
    pub fn into_scratch(self) -> Scratch {
        Scratch {
            caps: self.caps,
            group_starts: self.group_starts,
            counters: self.counters,
            iter_starts: self.iter_starts,
            stack: self.stack,
        }
    }

    /// Runs the pattern at `start`; on success the captures are in `caps`.
    pub fn match_at(&mut self, start: usize) -> bool {
        if let Some(firsts) = &self.prog.firsts
            && !firsts.iter().any(|f| match f {
                First::Item(item) => self.step(item, false, start).is_some(),
                First::InputStart => start == 0,
            })
        {
            return false;
        }
        self.caps.iter_mut().for_each(|c| *c = NONE);
        self.stack.clear();
        self.run(0, start).is_some()
    }

    /// The character after `pos` and the position after it.
    fn next_char(&self, pos: usize) -> Option<(u32, usize)> {
        let u = *self.input.get(pos)?;
        if self.prog.unicode
            && is_lead(u)
            && let Some(&t) = self.input.get(pos + 1)
            && is_trail(t)
        {
            let c = 0x10000 + (((u as u32) - 0xD800) << 10) + ((t as u32) - 0xDC00);
            return Some((c, pos + 2));
        }
        Some((u as u32, pos + 1))
    }

    /// The character before `pos` and the position before it.
    fn prev_char(&self, pos: usize) -> Option<(u32, usize)> {
        if pos == 0 {
            return None;
        }
        let u = self.input[pos - 1];
        if self.prog.unicode && is_trail(u) && pos >= 2 && is_lead(self.input[pos - 2]) {
            let l = self.input[pos - 2];
            let c = 0x10000 + (((l as u32) - 0xD800) << 10) + ((u as u32) - 0xDC00);
            return Some((c, pos - 2));
        }
        Some((u as u32, pos - 1))
    }

    fn item_matches(&self, item: &Item, c: u32) -> bool {
        match item {
            Item::Char { c: want, fold } => fold.canonicalize(c) == *want,
            Item::Set { set, invert, fold } => {
                kahvi_unicode::in_ranges(&self.prog.sets[*set], fold.canonicalize(c)) != *invert
            }
            Item::Any { dot_all } => *dot_all || !is_line_terminator(c),
        }
    }

    /// Matches one character in the given direction: the new position.
    fn step(&self, item: &Item, back: bool, pos: usize) -> Option<usize> {
        let (c, next) = if back {
            self.prev_char(pos)?
        } else {
            self.next_char(pos)?
        };
        self.item_matches(item, c).then_some(next)
    }

    /// Undoes one character step of a single-character loop that started at `start`.
    fn unstep(&self, back: bool, start: usize, pos: usize) -> usize {
        if back {
            // Undo a backward step: move forward, without passing `start`.
            if self.prog.unicode
                && pos + 2 <= start
                && is_lead(self.input[pos])
                && is_trail(self.input[pos + 1])
            {
                pos + 2
            } else {
                pos + 1
            }
        } else if self.prog.unicode
            && pos >= start + 2
            && is_trail(self.input[pos - 1])
            && is_lead(self.input[pos - 2])
        {
            pos - 2
        } else {
            pos - 1
        }
    }

    fn is_word_char(&self, word: usize, e: isize) -> bool {
        // IsWordChar(rer, Input, e): false at -1 and at InputLength.
        if e < 0 || e as usize >= self.input.len() {
            return false;
        }
        kahvi_unicode::in_ranges(&self.prog.sets[word], self.input[e as usize] as u32)
    }

    fn set_cap(&mut self, slot: usize, v: usize) {
        let old = self.caps[slot];
        if old != v {
            self.stack.push(Bt::Capture { slot, old });
            self.caps[slot] = v;
        }
    }

    /// Runs from `pc` until Match or LookEnd (Some(end position)) or until
    /// every choice made since entry has failed (None).
    fn run(&mut self, mut pc: usize, mut pos: usize) -> Option<usize> {
        let base = self.stack.len();
        loop {
            let ok = match &self.prog.insts[pc] {
                Inst::One { item, back } => match self.step(item, *back, pos) {
                    Some(next) => {
                        pos = next;
                        pc += 1;
                        true
                    }
                    None => false,
                },
                Inst::Start { multiline } => {
                    // ^: e = 0, or Multiline and Input[e - 1] is a line terminator.
                    let ok =
                        pos == 0 || (*multiline && is_line_terminator(self.input[pos - 1] as u32));
                    pc += 1;
                    ok
                }
                Inst::End { multiline } => {
                    let ok = pos == self.input.len()
                        || (*multiline && is_line_terminator(self.input[pos] as u32));
                    pc += 1;
                    ok
                }
                Inst::WordBoundary { negate, word } => {
                    let a = self.is_word_char(*word, pos as isize - 1);
                    let b = self.is_word_char(*word, pos as isize);
                    pc += 1;
                    (a != b) != *negate
                }
                Inst::Split { x, y } => {
                    self.stack.push(Bt::Branch { pc: *y, pos });
                    pc = *x;
                    true
                }
                Inst::Jump(to) => {
                    pc = *to;
                    true
                }
                Inst::GroupStart(group) => {
                    let group = *group;
                    self.stack.push(Bt::GroupStart {
                        group,
                        old: self.group_starts[group],
                    });
                    self.group_starts[group] = pos;
                    pc += 1;
                    true
                }
                Inst::GroupEnd { group, back } => {
                    let (group, back) = (*group, *back);
                    let other = self.group_starts[group];
                    // With direction backward the group was entered at its end.
                    let (s, e) = if back { (pos, other) } else { (other, pos) };
                    self.set_cap(group * 2, s);
                    self.set_cap(group * 2 + 1, e);
                    pc += 1;
                    true
                }
                Inst::ClearGroups { first, count } => {
                    for g in *first..*first + *count {
                        self.set_cap(g * 2, NONE);
                        self.set_cap(g * 2 + 1, NONE);
                    }
                    pc += 1;
                    true
                }
                Inst::BackRef { groups, fold, back } => {
                    let r = self.backreference(groups, *fold, *back, pos);
                    pc += 1;
                    match r {
                        Some(next) => {
                            pos = next;
                            true
                        }
                        None => false,
                    }
                }
                Inst::Look { negate, next } => {
                    let (negate, next) = (*negate, *next);
                    let snapshot = self.caps.clone();
                    let mark = self.stack.len();
                    let found = self.run(pc + 1, pos).is_some();
                    if found {
                        // Keep the lookaround's captures but none of its choice points;
                        // record how to undo the captures for when we backtrack past it.
                        self.stack.truncate(mark);
                        if negate {
                            self.caps = snapshot;
                        } else {
                            for (slot, &old) in snapshot.iter().enumerate() {
                                if self.caps[slot] != old {
                                    self.stack.push(Bt::Capture { slot, old });
                                }
                            }
                        }
                    }
                    pc = next;
                    found != negate
                }
                Inst::LookEnd | Inst::Match => return Some(pos),
                Inst::RepeatStart { reg } => {
                    let reg = *reg;
                    self.stack.push(Bt::Counter {
                        reg,
                        old: self.counters[reg],
                    });
                    self.counters[reg] = 0;
                    pc += 1;
                    true
                }
                Inst::RepeatHead {
                    reg,
                    min,
                    max,
                    greedy,
                    exit,
                } => {
                    let count = self.counters[*reg];
                    if count < *min {
                        self.enter_body(*reg, pos);
                        pc += 1;
                    } else if *max == Some(count) {
                        pc = *exit;
                    } else if *greedy {
                        // RepeatMatcher step 9-10: try the body, then the continuation.
                        self.stack.push(Bt::Branch { pc: *exit, pos });
                        self.enter_body(*reg, pos);
                        pc += 1;
                    } else {
                        // Step 8: try the continuation, then the body.
                        self.stack.push(Bt::EnterBody { head: pc, pos });
                        pc = *exit;
                    }
                    true
                }
                Inst::RepeatEnd { reg, min, head } => {
                    let reg = *reg;
                    let count = self.counters[reg];
                    // Step 2.b: if min = 0 and y's endIndex = x's endIndex, return failure.
                    if count >= *min && pos == self.iter_starts[reg] {
                        false
                    } else {
                        self.stack.push(Bt::Counter { reg, old: count });
                        self.counters[reg] = count.saturating_add(1);
                        pc = *head;
                        true
                    }
                }
                Inst::SimpleRepeat {
                    item,
                    back,
                    min,
                    max,
                    greedy,
                } => {
                    let (back, min, max) = (*back, *min, *max);
                    let start = pos;
                    let mut count = 0u64;
                    let limit = if *greedy { max } else { Some(min) };
                    let mut ok = true;
                    while limit.is_none_or(|l| count < l) {
                        match self.step(item, back, pos) {
                            Some(next) => {
                                pos = next;
                                count += 1;
                            }
                            None => break,
                        }
                    }
                    if count < min {
                        ok = false;
                    } else if *greedy {
                        if count > min {
                            self.stack.push(Bt::Greedy {
                                pc,
                                start,
                                pos,
                                count,
                                min,
                            });
                        }
                    } else if max.is_none_or(|m| count < m) {
                        self.stack.push(Bt::Lazy { pc, pos, count });
                    }
                    pc += 1;
                    ok
                }
            };
            if ok {
                continue;
            }
            // Backtrack to the most recent choice point.
            loop {
                if self.stack.len() == base {
                    return None;
                }
                match self.stack.pop().expect("non-empty stack") {
                    Bt::Branch { pc: p, pos: q } => {
                        pc = p;
                        pos = q;
                        break;
                    }
                    Bt::Capture { slot, old } => self.caps[slot] = old,
                    Bt::GroupStart { group, old } => self.group_starts[group] = old,
                    Bt::Counter { reg, old } => self.counters[reg] = old,
                    Bt::IterStart { reg, old } => self.iter_starts[reg] = old,
                    Bt::EnterBody { head, pos: q } => {
                        let Inst::RepeatHead { reg, .. } = self.prog.insts[head] else {
                            unreachable!("EnterBody points at a RepeatHead");
                        };
                        pos = q;
                        self.enter_body(reg, pos);
                        pc = head + 1;
                        break;
                    }
                    Bt::Greedy {
                        pc: p,
                        start,
                        pos: q,
                        count,
                        min,
                    } => {
                        let Inst::SimpleRepeat { back, .. } = self.prog.insts[p] else {
                            unreachable!("Greedy points at a SimpleRepeat");
                        };
                        let q = self.unstep(back, start, q);
                        if count - 1 > min {
                            self.stack.push(Bt::Greedy {
                                pc: p,
                                start,
                                pos: q,
                                count: count - 1,
                                min,
                            });
                        }
                        pc = p + 1;
                        pos = q;
                        break;
                    }
                    Bt::Lazy {
                        pc: p,
                        pos: q,
                        count,
                    } => {
                        let Inst::SimpleRepeat {
                            ref item,
                            back,
                            max,
                            ..
                        } = self.prog.insts[p]
                        else {
                            unreachable!("Lazy points at a SimpleRepeat");
                        };
                        if let Some(next) = self.step(item, back, q) {
                            if max.is_none_or(|m| count + 1 < m) {
                                self.stack.push(Bt::Lazy {
                                    pc: p,
                                    pos: next,
                                    count: count + 1,
                                });
                            }
                            pc = p + 1;
                            pos = next;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn enter_body(&mut self, reg: usize, pos: usize) {
        self.stack.push(Bt::IterStart {
            reg,
            old: self.iter_starts[reg],
        });
        self.iter_starts[reg] = pos;
    }

    /// BackreferenceMatcher(rer, ns, direction) (§22.2.2.7.2): the new position.
    fn backreference(&self, groups: &[usize], fold: Fold, back: bool, e: usize) -> Option<usize> {
        // The group of ns that participated, if any.
        let Some(&n) = groups.iter().find(|&&n| self.caps[n * 2] != NONE) else {
            // If s is undefined, return c(x).
            return Some(e);
        };
        let (s, se) = (self.caps[n * 2], self.caps[n * 2 + 1]);
        let len = se - s;
        // f = e + len (forward) or e - len (backward); fail if outside the input.
        let g = if back {
            e.checked_sub(len)?
        } else {
            if e + len > self.input.len() {
                return None;
            }
            e
        };
        let a = &self.input[s..se];
        let b = &self.input[g..g + len];
        let same = if fold == Fold::None {
            a == b
        } else {
            // Compare Canonicalize of each character.
            let (mut i, mut j) = (0, 0);
            let mut same = true;
            while i < len {
                let (ca, ni) = char_at(a, i, self.prog.unicode);
                let (cb, nj) = char_at(b, j, self.prog.unicode);
                if ni != nj || fold.canonicalize(ca) != fold.canonicalize(cb) {
                    same = false;
                    break;
                }
                i = ni;
                j = nj;
            }
            same
        };
        if !same {
            return None;
        }
        Some(if back { g } else { e + len })
    }
}

fn char_at(s: &[u16], i: usize, unicode: bool) -> (u32, usize) {
    let u = s[i];
    if unicode && is_lead(u) && i + 1 < s.len() && is_trail(s[i + 1]) {
        (
            0x10000 + (((u as u32) - 0xD800) << 10) + ((s[i + 1] as u32) - 0xDC00),
            i + 2,
        )
    } else {
        (u as u32, i + 1)
    }
}
