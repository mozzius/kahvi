//! The pattern grammar (§22.2.1) with its early errors (§22.2.1.1), and the
//! Annex B grammar (§B.1.2) for patterns without the `u` or `v` flag.
//!
//! The parser builds a tree whose character sets are already compiled
//! (CompileToCharSet, §22.2.2.9), because that depends on the RegExp Record
//! in force at each atom, which modifiers can change.

use crate::Error;
use crate::charset::{
    CharSet, PropertyError, Rer, character_complement, character_range, digits,
    maybe_simple_case_folding, property_set, white_space, word_characters,
};

#[derive(Clone, Debug)]
pub(crate) enum Node {
    Empty,
    /// A single character (PatternCharacter, a character escape).
    Char(u32),
    /// `.`
    Dot,
    /// A character class or class escape: its CharSet and whether it is inverted.
    Class(CharSet, bool),
    /// `^`
    Start,
    /// `$`
    End,
    /// `\b` (false) or `\B` (true).
    WordBoundary(bool),
    Look {
        behind: bool,
        negate: bool,
        body: Box<Node>,
    },
    /// A capturing group (index ≥ 1) or a non-capturing one.
    Group(Option<usize>, Box<Node>),
    /// `(?ims-ims: ...)`: the body runs with this RegExp Record.
    Modified(Rer, Box<Node>),
    /// A backreference to the groups with these indices (several for a
    /// duplicated name), resolved after parsing.
    BackRef(Vec<usize>),
    NamedBackRef(String),
    Alt(Vec<Node>),
    Seq(Vec<Node>),
    Repeat {
        body: Box<Node>,
        min: u64,
        max: Option<u64>,
        greedy: bool,
        /// The capturing groups inside the body: first index and count.
        first_group: usize,
        group_count: usize,
    },
}

/// The result of ParsePattern.
pub(crate) struct Parsed {
    pub node: Node,
    /// The number of capturing groups (not counting the whole match).
    pub group_count: usize,
    /// Each distinct group name with the indices of its groups, in order of appearance.
    pub group_names: Vec<(String, Vec<usize>)>,
}

/// One step of a group's path from the root: a Disjunction and which of its
/// Alternatives the group is in. Used for MightBothParticipate.
type PathStep = (u32, u32);

struct Parser<'a> {
    src: &'a [u32],
    pos: usize,
    rer: Rer,
    /// [UnicodeMode]
    unicode: bool,
    /// [NamedCaptureGroups]
    named_groups: bool,
    /// CountLeftCapturingParensWithin(the Pattern)
    total_groups: usize,
    group_count: usize,
    names: Vec<(String, usize, Vec<PathStep>)>,
    named_refs: Vec<String>,
    path: Vec<PathStep>,
    next_disjunction: u32,
}

const SYNTAX_CHARACTERS: &str = "^$\\.*+?()[]{}|";

fn is_syntax_character(c: u32) -> bool {
    char::from_u32(c).is_some_and(|c| SYNTAX_CHARACTERS.contains(c))
}

fn is_decimal_digit(c: u32) -> bool {
    (b'0' as u32..=b'9' as u32).contains(&c)
}

fn hex_value(c: u32) -> Option<u32> {
    char::from_u32(c).and_then(|c| c.to_digit(16))
}

fn is_lead(c: u32) -> bool {
    (0xD800..=0xDBFF).contains(&c)
}

fn is_trail(c: u32) -> bool {
    (0xDC00..=0xDFFF).contains(&c)
}

/// ParsePattern(patternText, u, v) (§22.2.3.4), with the Annex B change
/// (§B.1.2.9): without u or v, a pattern is first parsed without
/// [NamedCaptureGroups], and again with it if it contains a GroupName.
pub(crate) fn parse_pattern(src: &[u32], rer: Rer) -> Result<Parsed, Error> {
    let (total_groups, has_names) = prescan(src, rer.unicode_sets);
    let mut p = Parser {
        src,
        pos: 0,
        rer,
        unicode: rer.either_unicode(),
        named_groups: rer.either_unicode() || has_names,
        total_groups,
        group_count: 0,
        names: Vec::new(),
        named_refs: Vec::new(),
        path: Vec::new(),
        next_disjunction: 0,
    };
    let node = p.disjunction()?;
    if p.pos < src.len() {
        return Err(p.error(if p.peek() == Some(')' as u32) {
            "unmatched ')'"
        } else {
            "unexpected character"
        }));
    }
    p.finish(node)
}

/// CountLeftCapturingParensWithin and whether the pattern has a GroupName,
/// by a scan that skips escapes and character classes.
fn prescan(src: &[u32], sets_mode: bool) -> (usize, bool) {
    let mut count = 0;
    let mut has_names = false;
    let mut class_depth = 0;
    let mut i = 0;
    while i < src.len() {
        let c = src[i];
        if c == '\\' as u32 {
            i += 2;
            continue;
        }
        if class_depth > 0 {
            if c == ']' as u32 {
                class_depth -= 1;
            } else if c == '[' as u32 && sets_mode {
                class_depth += 1;
            }
        } else if c == '[' as u32 {
            class_depth = 1;
        } else if c == '(' as u32 {
            if src.get(i + 1) != Some(&('?' as u32)) {
                count += 1;
            } else if src.get(i + 2) == Some(&('<' as u32))
                && !matches!(src.get(i + 3).copied(), Some(x) if x == '=' as u32 || x == '!' as u32)
            {
                count += 1;
                has_names = true;
            }
        }
        i += 1;
    }
    (count, has_names)
}

impl Parser<'_> {
    fn error(&self, message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }

    fn peek(&self) -> Option<u32> {
        self.src.get(self.pos).copied()
    }

    fn peek_at(&self, n: usize) -> Option<u32> {
        self.src.get(self.pos + n).copied()
    }

    fn is(&self, c: char) -> bool {
        self.peek() == Some(c as u32)
    }

    fn eat(&mut self, c: char) -> bool {
        if self.is(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn looking_at(&self, s: &str) -> bool {
        s.chars()
            .enumerate()
            .all(|(i, c)| self.peek_at(i) == Some(c as u32))
    }

    fn expect(&mut self, c: char) -> Result<(), Error> {
        if self.eat(c) {
            Ok(())
        } else {
            Err(self.error(&format!("expected '{c}'")))
        }
    }

    // ---- Disjunction, Alternative, Term ------------------------------------

    fn disjunction(&mut self) -> Result<Node, Error> {
        let id = self.next_disjunction;
        self.next_disjunction += 1;
        let mut alternatives = Vec::new();
        let mut index = 0;
        loop {
            self.path.push((id, index));
            let alt = self.alternative();
            self.path.pop();
            alternatives.push(alt?);
            if !self.eat('|') {
                break;
            }
            index += 1;
        }
        Ok(if alternatives.len() == 1 {
            alternatives.pop().expect("one alternative")
        } else {
            Node::Alt(alternatives)
        })
    }

    fn alternative(&mut self) -> Result<Node, Error> {
        let mut terms = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' as u32 || c == ')' as u32 {
                break;
            }
            terms.push(self.term()?);
        }
        Ok(match terms.len() {
            0 => Node::Empty,
            1 => terms.pop().expect("one term"),
            _ => Node::Seq(terms),
        })
    }

    fn term(&mut self) -> Result<Node, Error> {
        let groups_before = self.group_count;
        // Assertions.
        if self.eat('^') {
            return Ok(Node::Start);
        }
        if self.eat('$') {
            return Ok(Node::End);
        }
        if self.looking_at("\\b") {
            self.pos += 2;
            return Ok(Node::WordBoundary(false));
        }
        if self.looking_at("\\B") {
            self.pos += 2;
            return Ok(Node::WordBoundary(true));
        }
        if self.looking_at("(?=")
            || self.looking_at("(?!")
            || self.looking_at("(?<=")
            || self.looking_at("(?<!")
        {
            let behind = self.peek_at(2) == Some('<' as u32);
            self.pos += if behind { 4 } else { 3 };
            let negate = self.src[self.pos - 1] == '!' as u32;
            let body = self.disjunction()?;
            self.expect(')')?;
            let node = Node::Look {
                behind,
                negate,
                body: Box::new(body),
            };
            // Lookbehinds are never quantifiable; lookaheads are only in the
            // Annex B grammar (QuantifiableAssertion).
            if behind || self.unicode {
                if self.at_quantifier() {
                    return Err(self.error("nothing to repeat"));
                }
                return Ok(node);
            }
            return self.quantified(node, groups_before);
        }
        let atom = self.atom()?;
        self.quantified(atom, groups_before)
    }

    fn at_quantifier(&self) -> bool {
        match self.peek() {
            Some(c) if c == '*' as u32 || c == '+' as u32 || c == '?' as u32 => true,
            Some(c) if c == '{' as u32 => self.braced_quantifier_len().is_some(),
            _ => false,
        }
    }

    /// The length of a `{n}`, `{n,}` or `{n,m}` at the current position.
    fn braced_quantifier_len(&self) -> Option<usize> {
        let mut i = 1;
        let digits_from = |i: &mut usize| {
            let start = *i;
            while self.peek_at(*i).is_some_and(is_decimal_digit) {
                *i += 1;
            }
            *i > start
        };
        if !digits_from(&mut i) {
            return None;
        }
        if self.peek_at(i) == Some(',' as u32) {
            i += 1;
            digits_from(&mut i);
        }
        if self.peek_at(i) == Some('}' as u32) {
            Some(i + 1)
        } else {
            None
        }
    }

    fn quantified(&mut self, atom: Node, groups_before: usize) -> Result<Node, Error> {
        let (min, max) = match self.peek() {
            Some(c) if c == '*' as u32 => {
                self.pos += 1;
                (0, None)
            }
            Some(c) if c == '+' as u32 => {
                self.pos += 1;
                (1, None)
            }
            Some(c) if c == '?' as u32 => {
                self.pos += 1;
                (0, Some(1))
            }
            Some(c) if c == '{' as u32 => match self.braced_quantifier_len() {
                Some(len) => {
                    let text: String = self.src[self.pos + 1..self.pos + len - 1]
                        .iter()
                        .map(|&c| char::from_u32(c).unwrap_or('?'))
                        .collect();
                    self.pos += len;
                    let (lo, hi) = match text.split_once(',') {
                        None => (text.as_str(), Some(text.as_str())),
                        Some((lo, "")) => (lo, None),
                        Some((lo, hi)) => (lo, Some(hi)),
                    };
                    // It is a Syntax Error if the MV of the first DecimalDigits is
                    // strictly greater than the MV of the second DecimalDigits.
                    if let Some(hi) = hi
                        && decimal_greater(lo, hi)
                    {
                        return Err(self.error("numbers out of order in {} quantifier"));
                    }
                    (decimal_value(lo), hi.map(decimal_value))
                }
                None => return Ok(atom),
            },
            _ => return Ok(atom),
        };
        let greedy = !self.eat('?');
        Ok(Node::Repeat {
            body: Box::new(atom),
            min,
            max,
            greedy,
            first_group: groups_before + 1,
            group_count: self.group_count - groups_before,
        })
    }

    fn atom(&mut self) -> Result<Node, Error> {
        let c = self.peek().expect("term at a character");
        match char::from_u32(c) {
            Some('.') => {
                self.pos += 1;
                Ok(Node::Dot)
            }
            Some('(') => self.group(),
            Some('[') => {
                let (set, invert) = self.character_class()?;
                Ok(Node::Class(set, invert))
            }
            Some('\\') => {
                self.pos += 1;
                self.atom_escape()
            }
            Some('*' | '+' | '?') => Err(self.error("nothing to repeat")),
            Some('{') => {
                if self.unicode {
                    return Err(self.error("lone quantifier brackets"));
                }
                // Annex B: InvalidBracedQuantifier is an error; otherwise `{`
                // is an ExtendedPatternCharacter.
                if self.braced_quantifier_len().is_some() {
                    return Err(self.error("nothing to repeat"));
                }
                self.pos += 1;
                Ok(Node::Char(c))
            }
            Some('}' | ']') if self.unicode => Err(self.error("lone quantifier brackets")),
            Some(')' | '|') => unreachable!("handled by alternative"),
            _ => {
                self.pos += 1;
                Ok(Node::Char(c))
            }
        }
    }

    fn group(&mut self) -> Result<Node, Error> {
        self.pos += 1; // (
        if self.eat('?') {
            if self.eat(':') {
                let body = self.disjunction()?;
                self.expect(')')?;
                return Ok(Node::Group(None, Box::new(body)));
            }
            if self.is('<') {
                self.pos += 1;
                let name = self.group_name()?;
                self.group_count += 1;
                let index = self.group_count;
                self.names.push((name, index, self.path.clone()));
                let body = self.disjunction()?;
                self.expect(')')?;
                return Ok(Node::Group(Some(index), Box::new(body)));
            }
            return self.modifiers();
        }
        self.group_count += 1;
        let index = self.group_count;
        let body = self.disjunction()?;
        self.expect(')')?;
        Ok(Node::Group(Some(index), Box::new(body)))
    }

    /// `(?` RegularExpressionModifiers [`-` RegularExpressionModifiers] `:` Disjunction `)`
    fn modifiers(&mut self) -> Result<Node, Error> {
        let mut add = String::new();
        let mut remove = String::new();
        let mut removing = false;
        loop {
            match self.peek().and_then(char::from_u32) {
                Some(c @ ('i' | 'm' | 's')) => {
                    self.pos += 1;
                    let list = if removing { &mut remove } else { &mut add };
                    // It is a Syntax Error if the source text matched by RegularExpressionModifiers
                    // contains the same code point more than once.
                    if list.contains(c) {
                        return Err(self.error("repeated flag in modifiers"));
                    }
                    list.push(c);
                }
                Some('-') if !removing => {
                    self.pos += 1;
                    removing = true;
                }
                Some(':') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(self.error("invalid group")),
            }
        }
        // It is a Syntax Error if the source text matched by the first RegularExpressionModifiers
        // and the source text matched by the second RegularExpressionModifiers are both empty.
        if removing && add.is_empty() && remove.is_empty() {
            return Err(self.error("invalid modifiers"));
        }
        // It is a Syntax Error if any code point in the first also occurs in the second.
        if add.chars().any(|c| remove.contains(c)) {
            return Err(self.error("flag both added and removed by modifiers"));
        }
        // UpdateModifiers(rer, add, remove) (§22.2.2.8)
        let saved = self.rer;
        for (list, on) in [(&add, true), (&remove, false)] {
            for c in list.chars() {
                match c {
                    'i' => self.rer.ignore_case = on,
                    'm' => self.rer.multiline = on,
                    _ => self.rer.dot_all = on,
                }
            }
        }
        let body = self.disjunction();
        let rer = self.rer;
        self.rer = saved;
        let body = body?;
        self.expect(')')?;
        Ok(Node::Modified(rer, Box::new(body)))
    }

    /// GroupName :: `<` RegExpIdentifierName `>` (after the `<`)
    fn group_name(&mut self) -> Result<String, Error> {
        let mut name = String::new();
        loop {
            if self.eat('>') {
                break;
            }
            let c = match self.peek() {
                None => return Err(self.error("invalid capture group name")),
                Some(c) if c == '\\' as u32 => {
                    self.pos += 1;
                    if !self.eat('u') {
                        return Err(self.error("invalid capture group name"));
                    }
                    // RegExpUnicodeEscapeSequence[+UnicodeMode]
                    match self.unicode_escape_body(true)? {
                        Some(c) => c,
                        None => return Err(self.error("invalid Unicode escape in group name")),
                    }
                }
                Some(c) => {
                    self.pos += 1;
                    // [~UnicodeMode] UnicodeLeadSurrogate UnicodeTrailSurrogate
                    if !self.unicode && is_lead(c) && self.peek().is_some_and(is_trail) {
                        let t = self.peek().expect("trail");
                        self.pos += 1;
                        0x10000 + ((c - 0xD800) << 10) + (t - 0xDC00)
                    } else {
                        c
                    }
                }
            };
            let ch = char::from_u32(c).ok_or_else(|| self.error("invalid capture group name"))?;
            let ok = if name.is_empty() {
                ch == '$' || ch == '_' || kahvi_unicode::is_id_start(ch)
            } else {
                ch == '$'
                    || ch == '\u{200C}'
                    || ch == '\u{200D}'
                    || kahvi_unicode::is_id_continue(ch)
            };
            if !ok {
                return Err(self.error("invalid capture group name"));
            }
            name.push(ch);
        }
        if name.is_empty() {
            return Err(self.error("invalid capture group name"));
        }
        Ok(name)
    }

    // ---- Escapes -----------------------------------------------------------

    /// AtomEscape, after the `\`.
    fn atom_escape(&mut self) -> Result<Node, Error> {
        let Some(c) = self.peek() else {
            return Err(self.error("\\ at end of pattern"));
        };
        // DecimalEscape
        if (b'1' as u32..=b'9' as u32).contains(&c) {
            let save = self.pos;
            let mut n: usize = 0;
            while let Some(d) = self.peek().filter(|&d| is_decimal_digit(d)) {
                n = n
                    .saturating_mul(10)
                    .saturating_add((d - '0' as u32) as usize);
                self.pos += 1;
            }
            if n <= self.total_groups {
                return Ok(Node::BackRef(vec![n]));
            }
            if self.unicode {
                // It is a Syntax Error if the CapturingGroupNumber of DecimalEscape is strictly
                // greater than CountLeftCapturingParensWithin(the Pattern).
                return Err(self.error("invalid escape"));
            }
            // Annex B: not a back reference, so a legacy octal or identity escape.
            self.pos = save;
        }
        if c == 'k' as u32 && self.named_groups {
            self.pos += 1;
            if !self.eat('<') {
                return Err(self.error("invalid named reference"));
            }
            let name = self.group_name()?;
            self.named_refs.push(name.clone());
            return Ok(Node::NamedBackRef(name));
        }
        if let Some(set) = self.character_class_escape()? {
            return Ok(Node::Class(set, false));
        }
        Ok(Node::Char(self.character_escape(false)?))
    }

    /// CharacterClassEscape, after the `\`; `None` if the next character does not start one.
    fn character_class_escape(&mut self) -> Result<Option<CharSet>, Error> {
        let Some(c) = self.peek().and_then(char::from_u32) else {
            return Ok(None);
        };
        let rer = self.rer;
        let set = match c {
            // CharacterClassEscape :: d — the ten digits.
            'd' => digits(),
            'D' => character_complement(rer, &digits()),
            's' => white_space(),
            'S' => character_complement(rer, &white_space()),
            'w' => maybe_simple_case_folding(rer, word_characters(rer)),
            'W' => character_complement(rer, &maybe_simple_case_folding(rer, word_characters(rer))),
            'p' | 'P' if self.unicode => {
                self.pos += 1;
                if !self.eat('{') {
                    return Err(self.error("invalid property name"));
                }
                let (name, value) = self.property_expression()?;
                let set =
                    property_set(&name, value.as_deref(), self.rer.unicode_sets).map_err(|e| {
                        match e {
                            PropertyError::Unknown => self.error("invalid property name"),
                            PropertyError::StringsWithoutV => {
                                self.error("property of strings needs the v flag")
                            }
                        }
                    })?;
                if c == 'P' {
                    // It is a Syntax Error if MayContainStrings of the UnicodePropertyValueExpression is true.
                    if set.has_strings() {
                        return Err(self.error("negated property of strings"));
                    }
                    return Ok(Some(character_complement(
                        rer,
                        &maybe_simple_case_folding(rer, set),
                    )));
                }
                return Ok(Some(maybe_simple_case_folding(rer, set)));
            }
            _ => return Ok(None),
        };
        self.pos += 1;
        Ok(Some(set))
    }

    /// UnicodePropertyValueExpression `}`
    fn property_expression(&mut self) -> Result<(String, Option<String>), Error> {
        let mut name = String::new();
        let mut value = None;
        loop {
            let Some(c) = self.peek().and_then(char::from_u32) else {
                return Err(self.error("invalid property name"));
            };
            self.pos += 1;
            match c {
                '}' => break,
                '=' if value.is_none() => value = Some(String::new()),
                c if c.is_ascii_alphabetic() || c == '_' => {
                    value.as_mut().unwrap_or(&mut name).push(c)
                }
                // UnicodePropertyValueCharacter includes DecimalDigit; UnicodePropertyNameCharacter does not.
                c if c.is_ascii_digit() => match &mut value {
                    Some(v) => v.push(c),
                    None => name.push(c),
                },
                _ => return Err(self.error("invalid property name")),
            }
        }
        if name.is_empty() || value.as_ref().is_some_and(|v| v.is_empty()) {
            return Err(self.error("invalid property name"));
        }
        if value.is_some() && name.chars().any(|c| c.is_ascii_digit()) {
            return Err(self.error("invalid property name"));
        }
        Ok((name, value))
    }

    /// CharacterEscape (and the Annex B additions), after the `\`; returns its CharacterValue.
    fn character_escape(&mut self, in_class: bool) -> Result<u32, Error> {
        let Some(c) = self.peek() else {
            return Err(self.error("\\ at end of pattern"));
        };
        self.pos += 1;
        let ch = char::from_u32(c);
        match ch {
            // ControlEscape
            Some('f') => return Ok(0x0C),
            Some('n') => return Ok(0x0A),
            Some('r') => return Ok(0x0D),
            Some('t') => return Ok(0x09),
            Some('v') => return Ok(0x0B),
            // `c` AsciiLetter
            Some('c') => {
                if let Some(l) = self
                    .peek()
                    .filter(|&l| char::from_u32(l).is_some_and(|l| l.is_ascii_alphabetic()))
                {
                    self.pos += 1;
                    return Ok(l % 32);
                }
                // Annex B: ClassControlLetter (a digit or `_`) inside a class.
                if !self.unicode
                    && in_class
                    && let Some(l) = self
                        .peek()
                        .filter(|&l| is_decimal_digit(l) || l == '_' as u32)
                {
                    self.pos += 1;
                    return Ok(l % 32);
                }
                if self.unicode {
                    return Err(self.error("invalid unicode escape"));
                }
                // Annex B: `\` [lookahead = c] is the character `\`.
                self.pos -= 1;
                return Ok('\\' as u32);
            }
            // `0` [lookahead ∉ DecimalDigit]
            Some('0') if !self.peek().is_some_and(is_decimal_digit) => return Ok(0),
            // HexEscapeSequence
            Some('x') => {
                if let (Some(a), Some(b)) = (
                    self.peek().and_then(hex_value),
                    self.peek_at(1).and_then(hex_value),
                ) {
                    self.pos += 2;
                    return Ok(a * 16 + b);
                }
                if self.unicode {
                    return Err(self.error("invalid escape"));
                }
                return Ok('x' as u32);
            }
            // RegExpUnicodeEscapeSequence
            Some('u') => {
                if let Some(v) = self.unicode_escape_body(self.unicode)? {
                    return Ok(v);
                }
                if self.unicode {
                    return Err(self.error("invalid unicode escape"));
                }
                return Ok('u' as u32);
            }
            _ => {}
        }
        if self.unicode {
            // IdentityEscape[+UnicodeMode] :: SyntaxCharacter or `/`; in a class also `-`.
            if is_syntax_character(c) || c == '/' as u32 || (in_class && c == '-' as u32) {
                return Ok(c);
            }
            return Err(self.error("invalid escape"));
        }
        // Annex B: LegacyOctalEscapeSequence
        if (b'0' as u32..=b'7' as u32).contains(&c) {
            let mut v = c - '0' as u32;
            let max_digits = if c <= '3' as u32 { 3 } else { 2 };
            let mut n = 1;
            while n < max_digits
                && let Some(d) = self
                    .peek()
                    .filter(|&d| (b'0' as u32..=b'7' as u32).contains(&d))
            {
                v = v * 8 + (d - '0' as u32);
                self.pos += 1;
                n += 1;
            }
            return Ok(v);
        }
        // Annex B: IdentityEscape :: SourceCharacterIdentityEscape
        if c == 'k' as u32 && self.named_groups {
            return Err(self.error("invalid escape"));
        }
        Ok(c)
    }

    /// After `\u`: the code point, or `None` if what follows is not a valid
    /// escape (which is an error in Unicode mode).
    fn unicode_escape_body(&mut self, unicode: bool) -> Result<Option<u32>, Error> {
        let hex4 = |p: &Parser, at: usize| -> Option<u32> {
            let mut v = 0;
            for i in 0..4 {
                v = v * 16 + p.peek_at(at + i).and_then(hex_value)?;
            }
            Some(v)
        };
        if unicode && self.is('{') {
            let save = self.pos;
            self.pos += 1;
            let mut v: u32 = 0;
            let mut digits = 0;
            while let Some(d) = self.peek().and_then(hex_value) {
                v = v.saturating_mul(16).saturating_add(d);
                digits += 1;
                self.pos += 1;
            }
            if digits > 0 && self.eat('}') && v <= 0x10FFFF {
                return Ok(Some(v));
            }
            self.pos = save;
            return Err(self.error("invalid unicode escape"));
        }
        let Some(v) = hex4(self, 0) else {
            return Ok(None);
        };
        self.pos += 4;
        // [+UnicodeMode] `u` HexLeadSurrogate `\u` HexTrailSurrogate
        if unicode
            && is_lead(v)
            && self.looking_at("\\u")
            && let Some(t) = hex4(self, 2)
            && is_trail(t)
        {
            self.pos += 6;
            return Ok(Some(0x10000 + ((v - 0xD800) << 10) + (t - 0xDC00)));
        }
        Ok(Some(v))
    }

    // ---- Character classes -------------------------------------------------

    /// CharacterClass: its CharSet and [[Invert]].
    fn character_class(&mut self) -> Result<(CharSet, bool), Error> {
        self.pos += 1; // [
        let negated = self.eat('^');
        if self.rer.unicode_sets {
            let set = self.class_set_expression()?;
            self.expect(']')?;
            if negated {
                // It is a Syntax Error if MayContainStrings of the ClassContents is true.
                if set.has_strings() {
                    return Err(self.error("negated character class may contain strings"));
                }
                return Ok((character_complement(self.rer, &set), false));
            }
            return Ok((set, false));
        }
        let set = self.class_ranges()?;
        Ok((set, negated))
    }

    /// NonemptyClassRanges up to and including the `]`.
    fn class_ranges(&mut self) -> Result<CharSet, Error> {
        let mut set = CharSet::empty();
        loop {
            if self.eat(']') {
                return Ok(set);
            }
            let a = self.class_atom()?;
            if self.is('-') && self.peek_at(1) != Some(']' as u32) && self.peek_at(1).is_some() {
                self.pos += 1;
                let b = self.class_atom()?;
                match (a, b) {
                    (ClassAtom::Char(x), ClassAtom::Char(y)) => {
                        // It is a Syntax Error if ... the CharacterValue of the first ClassAtom
                        // is strictly greater than the CharacterValue of the second ClassAtom.
                        if x > y {
                            return Err(self.error("range out of order in character class"));
                        }
                        set = set.union(&character_range(x, y));
                    }
                    (a, b) => {
                        // It is a Syntax Error if IsCharacterClass of either ClassAtom is true
                        // (in Unicode mode; Annex B makes it a union with `-` otherwise).
                        if self.unicode {
                            return Err(self.error("invalid character class range"));
                        }
                        set = set
                            .union(&a.into_set())
                            .union(&CharSet::char('-' as u32))
                            .union(&b.into_set());
                    }
                }
            } else {
                set = set.union(&a.into_set());
            }
            if self.peek().is_none() {
                return Err(self.error("unterminated character class"));
            }
        }
    }

    fn class_atom(&mut self) -> Result<ClassAtom, Error> {
        let Some(c) = self.peek() else {
            return Err(self.error("unterminated character class"));
        };
        self.pos += 1;
        if c != '\\' as u32 {
            return Ok(ClassAtom::Char(c));
        }
        // ClassEscape
        if self.eat('b') {
            return Ok(ClassAtom::Char(0x08));
        }
        if self.unicode && self.eat('-') {
            return Ok(ClassAtom::Char('-' as u32));
        }
        if let Some(set) = self.character_class_escape()? {
            return Ok(ClassAtom::Class(set));
        }
        // Annex B: in a class, a DecimalEscape is never a back reference.
        if !self.unicode
            && self
                .peek()
                .is_some_and(|d| d == '8' as u32 || d == '9' as u32)
        {
            let d = self.peek().expect("digit");
            self.pos += 1;
            return Ok(ClassAtom::Char(d));
        }
        if self.unicode
            && self
                .peek()
                .is_some_and(|d| (b'1' as u32..=b'9' as u32).contains(&d))
        {
            return Err(self.error("invalid class escape"));
        }
        Ok(ClassAtom::Char(self.character_escape(true)?))
    }

    // ---- ClassSetExpression (the v flag) -------------------------------------

    /// ClassSetExpression, up to (not including) the closing `]`.
    fn class_set_expression(&mut self) -> Result<CharSet, Error> {
        if self.is(']') {
            return Ok(CharSet::empty());
        }
        let first = self.class_set_operand_or_range()?;
        if self.looking_at("&&") {
            let Operand::Set(mut acc) = first else {
                return Err(self.error("invalid set operation"));
            };
            while self.looking_at("&&") {
                self.pos += 2;
                if self.is('&') {
                    return Err(self.error("invalid set operation"));
                }
                let Operand::Set(next) = self.class_set_operand_or_range()? else {
                    return Err(self.error("invalid set operation"));
                };
                acc = acc.intersection(&next);
            }
            if !self.is(']') {
                return Err(self.error("invalid set operation"));
            }
            return Ok(acc);
        }
        if self.looking_at("--") {
            let Operand::Set(mut acc) = first else {
                return Err(self.error("invalid set operation"));
            };
            while self.looking_at("--") {
                self.pos += 2;
                let Operand::Set(next) = self.class_set_operand_or_range()? else {
                    return Err(self.error("invalid set operation"));
                };
                acc = acc.subtract(&next);
            }
            if !self.is(']') {
                return Err(self.error("invalid set operation"));
            }
            return Ok(acc);
        }
        // ClassUnion
        let mut acc = first.into_set();
        while !self.is(']') {
            if self.peek().is_none() {
                return Err(self.error("unterminated character class"));
            }
            if self.looking_at("&&") || self.looking_at("--") {
                return Err(self.error("invalid set operation"));
            }
            acc = acc.union(&self.class_set_operand_or_range()?.into_set());
        }
        Ok(acc)
    }

    /// A ClassSetOperand, or a ClassSetRange (which only a ClassUnion allows).
    fn class_set_operand_or_range(&mut self) -> Result<Operand, Error> {
        let rer = self.rer;
        if self.is('[') {
            self.pos += 1;
            let negated = self.eat('^');
            let set = self.class_set_expression()?;
            self.expect(']')?;
            if negated {
                if set.has_strings() {
                    return Err(self.error("negated character class may contain strings"));
                }
                return Ok(Operand::Set(character_complement(rer, &set)));
            }
            return Ok(Operand::Set(set));
        }
        if self.looking_at("\\q{") {
            self.pos += 3;
            let mut set = CharSet::empty();
            loop {
                let mut s = Vec::new();
                while !self.is('|') && !self.is('}') {
                    if self.peek().is_none() {
                        return Err(self.error("unterminated class string disjunction"));
                    }
                    s.push(self.class_set_character()?);
                }
                set = set.union(&CharSet::string(s));
                if self.eat('}') {
                    break;
                }
                self.pos += 1; // |
            }
            return Ok(Operand::Set(maybe_simple_case_folding(rer, set)));
        }
        if self.is('\\') {
            self.pos += 1;
            if let Some(set) = self.character_class_escape()? {
                return Ok(Operand::Set(set));
            }
            self.pos -= 1;
        }
        let a = self.class_set_character()?;
        if self.is('-') && !self.looking_at("--") {
            self.pos += 1;
            let b = self.class_set_character()?;
            // It is a Syntax Error if the CharacterValue of the first ClassSetCharacter is
            // strictly greater than the CharacterValue of the second ClassSetCharacter.
            if a > b {
                return Err(self.error("range out of order in character class"));
            }
            return Ok(Operand::Range(maybe_simple_case_folding(
                rer,
                character_range(a, b),
            )));
        }
        Ok(Operand::Set(maybe_simple_case_folding(
            rer,
            CharSet::char(a),
        )))
    }

    /// ClassSetCharacter: its CharacterValue.
    fn class_set_character(&mut self) -> Result<u32, Error> {
        const RESERVED_DOUBLE: &str = "&!#$%*+,.:;<=>?@^`~";
        const SYNTAX: &str = "()[]{}/-\\|";
        const RESERVED_PUNCTUATOR: &str = "&-!#%,:;<=>@`~";
        let Some(c) = self.peek() else {
            return Err(self.error("unterminated character class"));
        };
        if c == '\\' as u32 {
            self.pos += 1;
            if self.eat('b') {
                return Ok(0x08);
            }
            if let Some(p) = self
                .peek()
                .filter(|&p| char::from_u32(p).is_some_and(|p| RESERVED_PUNCTUATOR.contains(p)))
            {
                self.pos += 1;
                return Ok(p);
            }
            return self.character_escape(false);
        }
        let ch = char::from_u32(c);
        if ch.is_some_and(|ch| RESERVED_DOUBLE.contains(ch)) && self.peek_at(1) == Some(c) {
            return Err(self.error("invalid set operation in character class"));
        }
        if ch.is_some_and(|ch| SYNTAX.contains(ch)) {
            return Err(self.error("invalid character in character class"));
        }
        self.pos += 1;
        Ok(c)
    }

    // ---- After parsing -----------------------------------------------------

    fn finish(self, node: Node) -> Result<Parsed, Error> {
        // Group names: each name with its groups, in order of first appearance.
        let mut group_names: Vec<(String, Vec<usize>)> = Vec::new();
        for (i, (name, index, path)) in self.names.iter().enumerate() {
            // It is a Syntax Error if Pattern contains two distinct GroupSpecifiers x and y such that
            // the CapturingGroupName of x is the CapturingGroupName of y and MightBothParticipate(x, y) is true.
            for (other, _, other_path) in &self.names[..i] {
                if other == name && might_both_participate(path, other_path) {
                    return Err(self.error("duplicate capture group name"));
                }
            }
            match group_names.iter_mut().find(|(n, _)| n == name) {
                Some((_, indices)) => indices.push(*index),
                None => group_names.push((name.clone(), vec![*index])),
            }
        }
        // It is a Syntax Error if GroupSpecifiersThatMatch(GroupName) is empty.
        for name in &self.named_refs {
            if !group_names.iter().any(|(n, _)| n == name) {
                return Err(self.error("invalid named capture referenced"));
            }
        }
        let node = resolve_named_refs(node, &group_names);
        Ok(Parsed {
            node,
            group_count: self.group_count,
            group_names,
        })
    }
}

/// MightBothParticipate(x, y) (§22.2.1.2): false when x and y are in
/// different Alternatives of some Disjunction.
fn might_both_participate(a: &[PathStep], b: &[PathStep]) -> bool {
    for (x, y) in a.iter().zip(b) {
        if x.0 != y.0 {
            return true;
        }
        if x.1 != y.1 {
            return false;
        }
    }
    true
}

fn resolve_named_refs(node: Node, names: &[(String, Vec<usize>)]) -> Node {
    let r = |n: Box<Node>| Box::new(resolve_named_refs(*n, names));
    match node {
        Node::NamedBackRef(name) => {
            let indices = names
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, i)| i.clone())
                .unwrap_or_default();
            Node::BackRef(indices)
        }
        Node::Look {
            behind,
            negate,
            body,
        } => Node::Look {
            behind,
            negate,
            body: r(body),
        },
        Node::Group(i, body) => Node::Group(i, r(body)),
        Node::Modified(rer, body) => Node::Modified(rer, r(body)),
        Node::Alt(v) => Node::Alt(
            v.into_iter()
                .map(|n| resolve_named_refs(n, names))
                .collect(),
        ),
        Node::Seq(v) => Node::Seq(
            v.into_iter()
                .map(|n| resolve_named_refs(n, names))
                .collect(),
        ),
        Node::Repeat {
            body,
            min,
            max,
            greedy,
            first_group,
            group_count,
        } => Node::Repeat {
            body: r(body),
            min,
            max,
            greedy,
            first_group,
            group_count,
        },
        other => other,
    }
}

enum ClassAtom {
    Char(u32),
    Class(CharSet),
}

impl ClassAtom {
    fn into_set(self) -> CharSet {
        match self {
            ClassAtom::Char(c) => CharSet::char(c),
            ClassAtom::Class(s) => s,
        }
    }
}

enum Operand {
    Set(CharSet),
    Range(CharSet),
}

impl Operand {
    fn into_set(self) -> CharSet {
        match self {
            Operand::Set(s) | Operand::Range(s) => s,
        }
    }
}

/// a > b for two DecimalDigits strings, compared by mathematical value.
fn decimal_greater(a: &str, b: &str) -> bool {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    (a.len(), a) > (b.len(), b)
}

/// The MV of DecimalDigits, saturating at u64::MAX (no input is that long).
fn decimal_value(s: &str) -> u64 {
    s.bytes().fold(0u64, |v, d| {
        v.saturating_mul(10).saturating_add((d - b'0') as u64)
    })
}
