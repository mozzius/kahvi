//! The lexer. The parser drives it token by token and tells it when a `/`
//! starts a regular expression or a `}` continues a template, since only the
//! grammar knows (ECMA-262 §12, "lexical goals").

use crate::SyntaxError;
use crate::ast::{Span, Utf16};
use crate::number::radix_digits_to_f64;
use kahvi_unicode::{is_id_continue, is_id_start};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum P {
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Ellipsis,
    Semi,
    Comma,
    Lt,
    Gt,
    LtEq,
    GtEq,
    EqEq,
    NotEq,
    EqEqEq,
    NotEqEq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    PlusPlus,
    MinusMinus,
    Shl,
    Shr,
    UShr,
    Amp,
    Pipe,
    Caret,
    Bang,
    Tilde,
    AmpAmp,
    PipePipe,
    QQ,
    Question,
    QDot,
    Colon,
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    StarStarEq,
    ShlEq,
    ShrEq,
    UShrEq,
    AmpEq,
    PipeEq,
    CaretEq,
    AmpAmpEq,
    PipePipeEq,
    QQEq,
    Arrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tok {
    Eof,
    /// Identifier or keyword; see `Token::ident` and `Token::escaped`.
    Ident,
    PrivateName,
    Punct(P),
    Number,
    BigInt,
    String,
    /// A template piece; `Token::template_tail` says whether it ended with a backtick.
    Template,
    RegExp,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Tok,
    pub start: usize,
    pub end: usize,
    /// A line terminator occurred between the previous token and this one.
    pub nl_before: bool,
    pub ident: String,
    /// The identifier was written with a unicode escape.
    pub escaped: bool,
    pub str_val: Utf16,
    pub raw: Utf16,
    /// Template: escapes were all valid (else the cooked value is undefined).
    pub cooked_ok: bool,
    pub template_tail: bool,
    pub num: f64,
    pub bigint: String,
    /// Legacy octal literal / octal or \8 \9 escape (errors in strict mode).
    pub legacy_octal: bool,
    pub regex_flags: String,
}

impl Token {
    pub(crate) fn new(kind: Tok, start: usize) -> Token {
        Token {
            kind,
            start,
            end: start,
            nl_before: false,
            ident: String::new(),
            escaped: false,
            str_val: Vec::new(),
            raw: Vec::new(),
            cooked_ok: true,
            template_tail: false,
            num: 0.0,
            bigint: String::new(),
            legacy_octal: false,
            regex_flags: String::new(),
        }
    }

    pub fn span(&self) -> Span {
        Span::new(self.start, self.end)
    }

    pub fn is(&self, p: P) -> bool {
        self.kind == Tok::Punct(p)
    }
}

pub fn is_line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\t' | '\u{0B}' | '\u{0C}' | ' ' | '\u{A0}' | '\u{FEFF}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

pub fn is_identifier_start(c: char) -> bool {
    c == '$' || c == '_' || is_id_start(c)
}

pub fn is_identifier_part(c: char) -> bool {
    c == '$' || c == '\u{200C}' || c == '\u{200D}' || is_id_continue(c)
}

#[derive(Clone)]
pub struct Lexer<'a> {
    src: &'a str,
    pub pos: usize,
    /// Byte offsets of the U+FFFD characters in `src` that stand for lone
    /// surrogates in the original UTF-16 source, with those code units.
    /// Sorted by offset.
    pub lone_surrogates: Vec<(usize, u16)>,
    /// Module goal: no HTML-like comments.
    pub module: bool,
}

type LexResult<T> = Result<T, SyntaxError>;

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, module: bool) -> Lexer<'a> {
        Lexer {
            src,
            pos: 0,
            lone_surrogates: Vec::new(),
            module,
        }
    }

    pub fn source(&self) -> &'a str {
        self.src
    }

    /// Appends the source character `c` that was just consumed: a lone
    /// surrogate keeps its own code unit.
    fn push_source_char(&self, buf: &mut Utf16, c: char) {
        self.push_source_char_at(buf, c, self.pos - c.len_utf8());
    }

    fn push_source_char_at(&self, buf: &mut Utf16, c: char, at: usize) {
        if c == '\u{FFFD}'
            && let Ok(i) = self.lone_surrogates.binary_search_by_key(&at, |&(p, _)| p)
        {
            buf.push(self.lone_surrogates[i].1);
        } else {
            push_char(buf, c);
        }
    }

    /// TRV of the template text `src[start..end]`: CR and CRLF become LF.
    fn push_raw(&self, raw: &mut Utf16, start: usize, end: usize) {
        let mut chars = self.src[start..end].char_indices().peekable();
        while let Some((i, c)) = chars.next() {
            if c == '\r' {
                if chars.peek().map(|&(_, n)| n) == Some('\n') {
                    chars.next();
                }
                raw.push(0x0A);
            } else {
                self.push_source_char_at(raw, c, start + i);
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_char_at(&self, offset: usize) -> Option<char> {
        self.src[self.pos..].chars().nth(offset)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek_char() == Some(c) {
            self.pos += c.len_utf8();
            true
        } else {
            false
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    fn err<T>(&self, msg: impl Into<String>) -> LexResult<T> {
        Err(SyntaxError::new(msg, self.pos))
    }

    fn err_at<T>(&self, msg: impl Into<String>, pos: usize) -> LexResult<T> {
        Err(SyntaxError::new(msg, pos))
    }

    /// Skips whitespace and comments. Returns whether a line terminator was seen.
    fn skip_trivia(&mut self) -> LexResult<bool> {
        let mut nl = false;
        // A hashbang comment is only allowed as the very first thing.
        if self.pos == 0 && self.starts_with("#!") {
            self.skip_line();
        }
        let at_start = self.pos == 0;
        let bytes = self.src.as_bytes();
        loop {
            // ASCII fast path: spaces, tabs and newlines, and the end of
            // trivia at any ASCII byte that cannot start a comment.
            match bytes.get(self.pos) {
                Some(b' ' | b'\t') => {
                    self.pos += 1;
                    continue;
                }
                Some(b'\n' | b'\r') => {
                    self.pos += 1;
                    nl = true;
                    continue;
                }
                Some(&b) if b < 0x80 && !matches!(b, b'/' | b'<' | b'-' | 0x0B | 0x0C) => break,
                None => break,
                _ => {}
            }
            let Some(c) = self.peek_char() else { break };
            if is_whitespace(c) {
                self.bump();
            } else if is_line_terminator(c) {
                self.bump();
                nl = true;
            } else if self.starts_with("//") {
                self.skip_line();
            } else if self.starts_with("/*") {
                let start = self.pos;
                self.pos += 2;
                loop {
                    match self.bump() {
                        None => return self.err_at("unterminated comment", start),
                        Some('*') if self.eat('/') => break,
                        Some(c) if is_line_terminator(c) => nl = true,
                        _ => {}
                    }
                }
            } else if !self.module
                && (self.starts_with("<!--") || (nl || at_start) && self.starts_with("-->"))
            {
                self.skip_line();
            } else {
                break;
            }
        }
        Ok(nl)
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.peek_char() {
            if is_line_terminator(c) {
                break;
            }
            self.bump();
        }
    }

    /// Lexes the next token. `/` is always a punctuator here; the parser calls
    /// [`Lexer::rescan_regexp`] where a regular expression is allowed.
    pub fn next_token(&mut self) -> LexResult<Token> {
        let nl = self.skip_trivia()?;
        let start = self.pos;
        let mut tok = match self.peek_char() {
            None => Token::new(Tok::Eof, start),
            Some(c) => self.lex_token(c, start)?,
        };
        tok.start = start;
        tok.end = self.pos;
        tok.nl_before = nl;
        Ok(tok)
    }

    fn lex_token(&mut self, c: char, start: usize) -> LexResult<Token> {
        if is_identifier_start(c) || c == '\\' {
            let (name, escaped) = self.lex_identifier_name()?;
            let mut t = Token::new(Tok::Ident, start);
            t.ident = name;
            t.escaped = escaped;
            return Ok(t);
        }
        if c.is_ascii_digit()
            || (c == '.' && self.peek_char_at(1).is_some_and(|d| d.is_ascii_digit()))
        {
            return self.lex_number(start);
        }
        match c {
            '"' | '\'' => return self.lex_string(c, start),
            '`' => {
                self.bump();
                return self.lex_template_part(start);
            }
            '#' => {
                self.bump();
                if !self
                    .peek_char()
                    .is_some_and(|c| is_identifier_start(c) || c == '\\')
                {
                    return self.err("invalid private name");
                }
                let (name, _) = self.lex_identifier_name()?;
                let mut t = Token::new(Tok::PrivateName, start);
                t.ident = name;
                return Ok(t);
            }
            _ => {}
        }
        let p = self.lex_punctuator()?;
        Ok(Token::new(Tok::Punct(p), start))
    }

    fn lex_punctuator(&mut self) -> LexResult<P> {
        use P::*;
        // Longest match first.
        const TABLE: &[(&str, P)] = &[
            (">>>=", UShrEq),
            ("...", Ellipsis),
            ("===", EqEqEq),
            ("!==", NotEqEq),
            ("**=", StarStarEq),
            ("<<=", ShlEq),
            (">>=", ShrEq),
            (">>>", UShr),
            ("&&=", AmpAmpEq),
            ("||=", PipePipeEq),
            ("??=", QQEq),
            ("=>", Arrow),
            ("==", EqEq),
            ("!=", NotEq),
            ("<=", LtEq),
            (">=", GtEq),
            ("&&", AmpAmp),
            ("||", PipePipe),
            ("??", QQ),
            ("++", PlusPlus),
            ("--", MinusMinus),
            ("**", StarStar),
            ("<<", Shl),
            (">>", Shr),
            ("+=", PlusEq),
            ("-=", MinusEq),
            ("*=", StarEq),
            ("/=", SlashEq),
            ("%=", PercentEq),
            ("&=", AmpEq),
            ("|=", PipeEq),
            ("^=", CaretEq),
            ("{", LBrace),
            ("}", RBrace),
            ("(", LParen),
            (")", RParen),
            ("[", LBracket),
            ("]", RBracket),
            (".", Dot),
            (";", Semi),
            (",", Comma),
            ("<", Lt),
            (">", Gt),
            ("+", Plus),
            ("-", Minus),
            ("*", Star),
            ("/", Slash),
            ("%", Percent),
            ("&", Amp),
            ("|", Pipe),
            ("^", Caret),
            ("!", Bang),
            ("~", Tilde),
            ("?", Question),
            (":", Colon),
            ("=", Eq),
        ];
        // `?.` is only optional chaining when not followed by a decimal digit.
        if self.starts_with("?.") && !self.peek_char_at(2).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 2;
            return Ok(QDot);
        }
        let first = self.src.as_bytes().get(self.pos).copied();
        // Single-character punctuators no longer one can start with.
        let single = match first {
            Some(b'{') => Some(LBrace),
            Some(b'}') => Some(RBrace),
            Some(b'(') => Some(LParen),
            Some(b')') => Some(RParen),
            Some(b'[') => Some(LBracket),
            Some(b']') => Some(RBracket),
            Some(b';') => Some(Semi),
            Some(b',') => Some(Comma),
            Some(b'~') => Some(Tilde),
            Some(b':') => Some(Colon),
            _ => None,
        };
        if let Some(p) = single {
            self.pos += 1;
            return Ok(p);
        }
        for &(s, p) in TABLE {
            if Some(s.as_bytes()[0]) == first && self.starts_with(s) {
                self.pos += s.len();
                return Ok(p);
            }
        }
        let c = self.peek_char().unwrap_or('\0');
        self.err(format!("unexpected character {c:?}"))
    }

    /// IdentifierName, decoding `\u` escapes. Returns (name, contained an escape).
    fn lex_identifier_name(&mut self) -> LexResult<(String, bool)> {
        // ASCII fast path: the leading run of [A-Za-z0-9_$] is copied at once
        // (the caller has checked the first character starts an identifier).
        let bytes = self.src.as_bytes();
        let begin = self.pos;
        let mut end = begin;
        while end < bytes.len()
            && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_' || bytes[end] == b'$')
        {
            end += 1;
        }
        let mut name = String::with_capacity(end - begin);
        name.push_str(&self.src[begin..end]);
        self.pos = end;
        if bytes.get(end).is_none_or(|&b| b < 0x80 && b != b'\\') {
            return Ok((name, false));
        }
        let mut escaped = false;
        let mut first = begin == end;
        while let Some(c) = self.peek_char() {
            let c = if c == '\\' {
                let at = self.pos;
                self.bump();
                if !self.eat('u') {
                    return self.err_at("invalid escape in identifier", at);
                }
                escaped = true;
                let cp = self.lex_unicode_escape_body()?;
                let ch = char::from_u32(cp)
                    .ok_or_else(|| SyntaxError::new("invalid identifier escape", at))?;
                let ok = if first {
                    is_identifier_start(ch)
                } else {
                    is_identifier_part(ch)
                };
                if !ok {
                    return self.err_at("invalid character in identifier escape", at);
                }
                ch
            } else {
                let ok = if first {
                    is_identifier_start(c)
                } else {
                    is_identifier_part(c)
                };
                if !ok {
                    break;
                }
                self.bump();
                c
            };
            name.push(c);
            first = false;
        }
        Ok((name, escaped))
    }

    /// After `\u`: `XXXX` or `{X...}`. Returns the code point.
    fn lex_unicode_escape_body(&mut self) -> LexResult<u32> {
        if self.eat('{') {
            let mut v: u32 = 0;
            let mut n = 0;
            while let Some(c) = self.peek_char() {
                if c == '}' {
                    break;
                }
                let d = c
                    .to_digit(16)
                    .ok_or_else(|| SyntaxError::new("invalid unicode escape", self.pos))?;
                v = v.saturating_mul(16).saturating_add(d);
                if v > 0x10FFFF {
                    return self.err("unicode escape out of range");
                }
                self.bump();
                n += 1;
            }
            if n == 0 || !self.eat('}') {
                return self.err("invalid unicode escape");
            }
            Ok(v)
        } else {
            let mut v = 0;
            for _ in 0..4 {
                let d = self
                    .peek_char()
                    .and_then(|c| c.to_digit(16))
                    .ok_or_else(|| SyntaxError::new("invalid unicode escape", self.pos))?;
                self.bump();
                v = v * 16 + d;
            }
            Ok(v)
        }
    }

    /// Digits with numeric separators. Returns the digits without separators.
    fn lex_digits(&mut self, radix: u32, allow_separators: bool) -> LexResult<String> {
        let mut out = String::new();
        let mut last_sep = false;
        while let Some(c) = self.peek_char() {
            if c == '_' {
                if !allow_separators || out.is_empty() || last_sep {
                    return self.err("invalid numeric separator");
                }
                last_sep = true;
                self.bump();
                continue;
            }
            if !c.is_digit(radix) {
                break;
            }
            out.push(c);
            last_sep = false;
            self.bump();
        }
        if last_sep {
            return self.err("numeric separator at end of digits");
        }
        Ok(out)
    }

    fn lex_number(&mut self, start: usize) -> LexResult<Token> {
        let mut tok = Token::new(Tok::Number, start);
        let c = self.peek_char().unwrap_or('0');
        let next = self.peek_char_at(1).map(|c| c.to_ascii_lowercase());
        if c == '0' && matches!(next, Some('x' | 'o' | 'b')) {
            let radix = match next {
                Some('x') => 16,
                Some('o') => 8,
                _ => 2,
            };
            self.pos += 2;
            let digits = self.lex_digits(radix, true)?;
            if digits.is_empty() {
                return self.err("missing digits after radix prefix");
            }
            if self.eat('n') {
                tok.kind = Tok::BigInt;
                tok.bigint = radix_to_decimal_string(&digits, radix);
            } else {
                tok.num = radix_digits_to_f64(&digits, radix);
            }
            return self.finish_number(tok);
        }
        if c == '0'
            && self
                .peek_char_at(1)
                .is_some_and(|d| d.is_ascii_digit() || d == '_')
        {
            // LegacyOctalIntegerLiteral or NonOctalDecimalIntegerLiteral (sloppy only).
            self.bump();
            if self.peek_char() == Some('_') {
                return self.err("numeric separator after leading zero");
            }
            let digits = self.lex_digits(10, false)?;
            tok.legacy_octal = true;
            if digits.chars().all(|d| d < '8') {
                tok.num = radix_digits_to_f64(&digits, 8);
                if self.peek_char() == Some('n') {
                    return self.err("BigInt literal cannot use legacy octal");
                }
                return self.finish_number(tok);
            }
            // NonOctalDecimalIntegerLiteral: may continue as a decimal literal.
            let mut text = digits;
            self.lex_fraction_and_exponent(&mut text)?;
            if self.peek_char() == Some('n') {
                return self.err("invalid BigInt literal");
            }
            tok.num = text.parse().unwrap_or(f64::NAN);
            return self.finish_number(tok);
        }
        let mut text = if c == '.' {
            String::new()
        } else {
            self.lex_digits(10, true)?
        };
        let integer_only =
            self.peek_char() != Some('.') && !matches!(self.peek_char(), Some('e' | 'E'));
        if integer_only && self.peek_char() == Some('n') {
            self.bump();
            tok.kind = Tok::BigInt;
            tok.bigint = text.trim_start_matches('0').to_string();
            if tok.bigint.is_empty() {
                tok.bigint = "0".into();
            }
            if text.len() > 1 && text.starts_with('0') {
                return self.err("invalid BigInt literal");
            }
            return self.finish_number(tok);
        }
        self.lex_fraction_and_exponent(&mut text)?;
        tok.num = if text.starts_with('.') {
            format!("0{text}")
        } else {
            text
        }
        .parse()
        .unwrap_or(f64::NAN);
        self.finish_number(tok)
    }

    fn lex_fraction_and_exponent(&mut self, text: &mut String) -> LexResult<()> {
        if self.eat('.') {
            text.push('.');
            if self.peek_char() == Some('_') {
                return self.err("numeric separator after decimal point");
            }
            text.push_str(&self.lex_digits(10, true)?);
        }
        if matches!(self.peek_char(), Some('e' | 'E')) {
            self.bump();
            text.push('e');
            if let Some(sign @ ('+' | '-')) = self.peek_char() {
                self.bump();
                text.push(sign);
            }
            let exp = self.lex_digits(10, true)?;
            if exp.is_empty() {
                return self.err("missing exponent");
            }
            text.push_str(&exp);
        }
        Ok(())
    }

    fn finish_number(&mut self, mut tok: Token) -> LexResult<Token> {
        // The source character immediately following a NumericLiteral must not
        // be an IdentifierStart or DecimalDigit.
        if let Some(c) = self.peek_char()
            && (is_identifier_start(c) || c.is_ascii_digit() || c == '\\')
        {
            return self.err("identifier starts immediately after numeric literal");
        }
        tok.end = self.pos;
        Ok(tok)
    }

    fn lex_string(&mut self, quote: char, start: usize) -> LexResult<Token> {
        let mut tok = Token::new(Tok::String, start);
        self.bump();
        let mut buf: Utf16 = Vec::new();
        let bytes = self.src.as_bytes();
        let q = quote as u8;
        loop {
            // ASCII fast path: bytes other than the quote, `\\` and newlines
            // are code units of the value.
            let run = self.pos;
            while let Some(&b) = bytes.get(self.pos) {
                if b >= 0x80 || b == q || b == b'\\' || b == b'\n' || b == b'\r' {
                    break;
                }
                self.pos += 1;
            }
            buf.extend(bytes[run..self.pos].iter().map(|&b| b as u16));
            let Some(c) = self.bump() else {
                return self.err_at("unterminated string literal", start);
            };
            match c {
                c if c == quote => break,
                '\\' => {
                    let r = self.lex_escape(false)?;
                    if r.legacy_octal {
                        tok.legacy_octal = true;
                    }
                    match r.value {
                        Some(v) => buf.extend(v),
                        None => return self.err("invalid escape sequence"),
                    }
                }
                '\n' | '\r' => return self.err_at("unterminated string literal", start),
                c => self.push_source_char(&mut buf, c),
            }
        }
        tok.str_val = buf;
        Ok(tok)
    }

    /// After a backslash in a string or template. `value` is None for an
    /// escape that is invalid (only allowed, as undefined, in tagged templates).
    fn lex_escape(&mut self, template: bool) -> LexResult<Escape> {
        let Some(c) = self.bump() else {
            return self.err("unterminated escape");
        };
        let one = |v: u16| Escape {
            value: Some(vec![v]),
            legacy_octal: false,
        };
        Ok(match c {
            'n' => one(0x0A),
            't' => one(0x09),
            'r' => one(0x0D),
            'b' => one(0x08),
            'f' => one(0x0C),
            'v' => one(0x0B),
            '\r' => {
                self.eat('\n');
                Escape {
                    value: Some(vec![]),
                    legacy_octal: false,
                }
            }
            c if is_line_terminator(c) => Escape {
                value: Some(vec![]),
                legacy_octal: false,
            },
            '0' if !self.peek_char().is_some_and(|d| d.is_ascii_digit()) => one(0),
            '0'..='7' => {
                if template {
                    return Ok(Escape {
                        value: None,
                        legacy_octal: true,
                    });
                }
                // LegacyOctalEscapeSequence: up to three digits, value ≤ 0o377.
                let mut v = c.to_digit(8).unwrap_or(0);
                let max_digits = if c <= '3' { 3 } else { 2 };
                for _ in 1..max_digits {
                    match self.peek_char().and_then(|d| d.to_digit(8)) {
                        Some(d) => {
                            v = v * 8 + d;
                            self.bump();
                        }
                        None => break,
                    }
                }
                Escape {
                    value: Some(vec![v as u16]),
                    legacy_octal: true,
                }
            }
            '8' | '9' => {
                if template {
                    return Ok(Escape {
                        value: None,
                        legacy_octal: true,
                    });
                }
                Escape {
                    value: Some(vec![c as u16]),
                    legacy_octal: true,
                }
            }
            'x' => {
                let hi = self.peek_char().and_then(|c| c.to_digit(16));
                let lo = self.peek_char_at(1).and_then(|c| c.to_digit(16));
                match (hi, lo) {
                    (Some(h), Some(l)) => {
                        self.pos += 2;
                        one((h * 16 + l) as u16)
                    }
                    _ if template => Escape {
                        value: None,
                        legacy_octal: false,
                    },
                    _ => return self.err("invalid hexadecimal escape"),
                }
            }
            'u' => {
                let save = self.pos;
                match self.lex_unicode_escape_body() {
                    Ok(cp) => {
                        let mut v = Vec::new();
                        push_code_point(&mut v, cp);
                        Escape {
                            value: Some(v),
                            legacy_octal: false,
                        }
                    }
                    Err(e) if !template => return Err(e),
                    Err(_) => {
                        self.pos = save;
                        Escape {
                            value: None,
                            legacy_octal: false,
                        }
                    }
                }
            }
            c => {
                let mut v = Vec::new();
                self.push_source_char(&mut v, c);
                Escape {
                    value: Some(v),
                    legacy_octal: false,
                }
            }
        })
    }

    /// Scans a template piece starting just after `` ` `` or `}`.
    fn lex_template_part(&mut self, start: usize) -> LexResult<Token> {
        let mut tok = Token::new(Tok::Template, start);
        let mut cooked: Utf16 = Vec::new();
        let mut raw: Utf16 = Vec::new();
        loop {
            let Some(c) = self.bump() else {
                return self.err_at("unterminated template literal", start);
            };
            match c {
                '`' => {
                    tok.template_tail = true;
                    break;
                }
                '$' if self.eat('{') => break,
                '\\' => {
                    let esc_start = self.pos;
                    let r = self.lex_escape(true)?;
                    match r.value {
                        Some(v) => cooked.extend(v),
                        None => tok.cooked_ok = false,
                    }
                    raw.push('\\' as u16);
                    // Raw text of the escape, with line terminators normalised.
                    self.push_raw(&mut raw, esc_start, self.pos);
                }
                '\r' => {
                    self.eat('\n');
                    cooked.push(0x0A);
                    raw.push(0x0A);
                }
                c => {
                    self.push_source_char(&mut cooked, c);
                    self.push_source_char(&mut raw, c);
                }
            }
        }
        tok.str_val = cooked;
        tok.raw = raw;
        tok.end = self.pos;
        Ok(tok)
    }

    /// Re-lexes from a `}` (at `brace`) as the continuation of a template.
    pub fn rescan_template_continuation(&mut self, brace: usize) -> LexResult<Token> {
        self.pos = brace + 1;
        let mut tok = self.lex_template_part(brace)?;
        tok.start = brace;
        Ok(tok)
    }

    /// Re-lexes from a `/` (at `start`) as a RegularExpressionLiteral.
    pub fn rescan_regexp(&mut self, start: usize, nl_before: bool) -> LexResult<Token> {
        self.pos = start + 1;
        let mut in_class = false;
        let mut body: Utf16 = Vec::new();
        loop {
            let Some(c) = self.bump() else {
                return self.err_at("unterminated regular expression", start);
            };
            if is_line_terminator(c) {
                return self.err_at("unterminated regular expression", start);
            }
            match c {
                '\\' => {
                    body.push(c as u16);
                    match self.bump() {
                        Some(n) if !is_line_terminator(n) => self.push_source_char(&mut body, n),
                        _ => return self.err_at("unterminated regular expression", start),
                    }
                    continue;
                }
                '[' => in_class = true,
                ']' => in_class = false,
                '/' if !in_class => break,
                _ => {}
            }
            self.push_source_char(&mut body, c);
        }
        if body.is_empty() {
            return self.err_at("empty regular expression", start);
        }
        let mut flags = String::new();
        while let Some(c) = self.peek_char() {
            if c == '\\' {
                return self.err("escapes are not allowed in regular expression flags");
            }
            if !is_identifier_part(c) {
                break;
            }
            flags.push(c);
            self.bump();
        }
        let mut tok = Token::new(Tok::RegExp, start);
        tok.str_val = body;
        tok.regex_flags = flags;
        tok.end = self.pos;
        tok.nl_before = nl_before;
        Ok(tok)
    }
}

struct Escape {
    value: Option<Utf16>,
    legacy_octal: bool,
}

fn push_char(buf: &mut Utf16, c: char) {
    let mut tmp = [0u16; 2];
    buf.extend_from_slice(c.encode_utf16(&mut tmp));
}

fn push_code_point(buf: &mut Utf16, cp: u32) {
    match char::from_u32(cp) {
        Some(c) => push_char(buf, c),
        // A lone surrogate written as \uD800.
        None => buf.push(cp as u16),
    }
}

/// Converts digits in radix 2/8/16 to a decimal digit string (for BigInt literals).
fn radix_to_decimal_string(digits: &str, radix: u32) -> String {
    // Little-endian base 10^9 limbs.
    let mut limbs: Vec<u32> = vec![0];
    for c in digits.chars() {
        let mut carry = c.to_digit(radix).unwrap_or(0) as u64;
        for limb in limbs.iter_mut() {
            let v = *limb as u64 * radix as u64 + carry;
            *limb = (v % 1_000_000_000) as u32;
            carry = v / 1_000_000_000;
        }
        if carry > 0 {
            limbs.push(carry as u32);
        }
    }
    let mut out = limbs.last().copied().unwrap_or(0).to_string();
    for limb in limbs.iter().rev().skip(1) {
        out.push_str(&format!("{limb:09}"));
    }
    out
}
