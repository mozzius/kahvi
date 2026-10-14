//! Recursive-descent parser. Split by grammar area:
//! `stmt` (statements, declarations), `expr` (expressions), `func`
//! (functions, classes), `pattern` (destructuring), `module` (imports and
//! exports). Early errors are checked as constructs are parsed, plus the
//! declaration-conflict checks in [`crate::early`].

mod expr;
mod func;
mod module;
mod pattern;
mod stmt;

use std::collections::HashMap;

use crate::SyntaxError;
use crate::ast::*;
use crate::lexer::{Lexer, P, Tok, Token};

pub type PResult<T> = Result<T, SyntaxError>;

/// Context for code that is not a standalone script, e.g. direct `eval`.
#[derive(Debug, Clone)]
pub struct ParseOptions {
    pub strict: bool,
    pub allow_new_target: bool,
    pub allow_super_property: bool,
    pub allow_super_call: bool,
    pub allow_arguments: bool,
    /// Inside a class static block or field initializer (eval there).
    pub in_class_field_init: bool,
    /// Private names of enclosing classes (direct eval inside a class body).
    pub private_names: Vec<String>,
    /// Lone surrogates in the original UTF-16 source, which `src` holds as
    /// U+FFFD: (byte offset in `src`, code unit), sorted by offset.
    pub lone_surrogates: Vec<(usize, u16)>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            strict: false,
            allow_new_target: false,
            allow_super_property: false,
            allow_super_call: false,
            allow_arguments: true,
            in_class_field_init: false,
            private_names: Vec::new(),
            lone_surrogates: Vec::new(),
        }
    }
}

pub fn parse_script(src: &str, opts: &ParseOptions) -> PResult<Script> {
    let mut p = Parser::new(src, false, opts)?;
    p.strict = opts.strict;
    let body = p.parse_directives_and_statements(true, |p| p.tok.kind == Tok::Eof)?;
    let strict = p.strict;
    // ScriptBody : StatementList — It is a Syntax Error if ContainsUsing of StatementList is true.
    if let Some(at) = stmt::contains_using(&body) {
        return p.err(
            "'using' declarations are not allowed at the top level of a script",
            at,
        );
    }
    crate::early::check_script_declarations(&body, p.fn_ctx().is_eval_like)?;
    p.finish_private_names()?;
    Ok(Script {
        body,
        strict,
        span: Span::new(0, src.len()),
    })
}

/// The parsing of CreateDynamicFunction (§20.2.1.1.1 steps 17-24): `params`
/// and `body` must each parse alone before `source`, the whole function
/// expression, is parsed.
pub fn parse_dynamic_function(
    params: &str,
    body: &str,
    source: &str,
    is_async: bool,
    is_generator: bool,
) -> PResult<Function> {
    Parser::parse_dynamic_function_parts(params, body, source, is_async, is_generator)
}

pub fn parse_module(src: &str) -> PResult<Module> {
    let opts = ParseOptions {
        strict: true,
        ..ParseOptions::default()
    };
    let mut p = Parser::new(src, true, &opts)?;
    p.strict = true;
    p.fn_ctx_mut().is_async = true; // top-level await
    let body = p.parse_module_items()?;
    p.finish_private_names()?;
    crate::early::check_module(&body)?;
    Ok(Module {
        body,
        span: Span::new(0, src.len()),
    })
}

#[derive(Debug, Clone)]
pub(crate) struct Label {
    name: String,
    is_loop: bool,
}

/// Per-function parsing context.
#[derive(Debug, Clone)]
pub(crate) struct FnCtx {
    pub is_async: bool,
    pub is_generator: bool,
    pub is_arrow: bool,
    pub allow_return: bool,
    pub allow_new_target: bool,
    pub allow_super_property: bool,
    pub allow_super_call: bool,
    pub allow_arguments: bool,
    /// Class static block: `await` is reserved, `arguments` is not allowed.
    pub in_static_block: bool,
    /// Parsing formal parameters: yield/await expressions are errors.
    pub in_params: bool,
    pub is_eval_like: bool,
    pub labels: Vec<Label>,
    pub in_iteration: u32,
    pub in_switch: u32,
    pub uses_arguments: bool,
    pub uses_this: bool,
    pub has_direct_eval: bool,
    /// Position of the first yield/await expression seen while parsing
    /// arrow parameters (they are not allowed there).
    pub yield_or_await_pos: Option<usize>,
    /// Position of the first `await` used as an identifier in this function's
    /// own code (not nested function bodies): an async arrow head parsed as a
    /// call cover must not contain one.
    pub await_ident_pos: std::cell::Cell<Option<usize>>,
}

impl FnCtx {
    fn top(opts: &ParseOptions) -> FnCtx {
        FnCtx {
            is_async: false,
            is_generator: false,
            is_arrow: false,
            allow_return: false,
            allow_new_target: opts.allow_new_target,
            allow_super_property: opts.allow_super_property,
            allow_super_call: opts.allow_super_call,
            allow_arguments: opts.allow_arguments && !opts.in_class_field_init,
            in_static_block: false,
            in_params: false,
            is_eval_like: false,
            labels: Vec::new(),
            in_iteration: 0,
            in_switch: 0,
            uses_arguments: false,
            uses_this: false,
            has_direct_eval: false,
            yield_or_await_pos: None,
            await_ident_pos: Default::default(),
        }
    }
}

/// Private names declared in a class body, and references waiting to be
/// resolved when the body ends.
#[derive(Debug, Default)]
pub(crate) struct ClassScope {
    pub declared: HashMap<String, PrivateKind>,
    pub unresolved: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PrivateKind {
    Field,
    Method,
    Getter,
    Setter,
    GetterSetter,
    StaticField,
    StaticMethod,
    StaticGetter,
    StaticSetter,
    StaticGetterSetter,
}

pub(crate) struct Parser<'a> {
    pub lx: Lexer<'a>,
    pub tok: Token,
    pub prev_end: usize,
    pub strict: bool,
    pub module: bool,
    fns: Vec<FnCtx>,
    pub classes: Vec<ClassScope>,
    /// Private names visible from outside the source (eval in a class).
    outer_private: Vec<String>,
    /// The [In] grammar parameter.
    pub allow_in: bool,
    /// Array literals whose spread element was followed by a comma (invalid as patterns).
    pub spread_trailing_comma: std::collections::HashSet<usize>,
    /// The last statement list parsed by `parse_directives_and_statements`
    /// had a "use strict" directive.
    pub saw_use_strict: bool,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str, module: bool, opts: &ParseOptions) -> PResult<Parser<'a>> {
        let mut lx = Lexer::new(src, module);
        lx.lone_surrogates = opts.lone_surrogates.clone();
        let tok = lx.next_token()?;
        let mut top = FnCtx::top(opts);
        top.is_eval_like = opts.allow_new_target || opts.allow_super_property;
        Ok(Parser {
            lx,
            tok,
            prev_end: 0,
            strict: opts.strict,
            module,
            fns: vec![top],
            classes: Vec::new(),
            outer_private: opts.private_names.clone(),
            allow_in: true,
            spread_trailing_comma: Default::default(),
            saw_use_strict: false,
        })
    }

    // ---- tokens --------------------------------------------------------

    pub fn next(&mut self) -> PResult<Token> {
        let next = self.lx.next_token()?;
        self.prev_end = self.tok.end;
        Ok(std::mem::replace(&mut self.tok, next))
    }

    /// The token after the current one, without consuming anything.
    pub fn peek(&self) -> PResult<Token> {
        let mut lx = self.lx.clone();
        lx.next_token()
    }

    pub fn is(&self, p: P) -> bool {
        self.tok.kind == Tok::Punct(p)
    }

    pub fn eat(&mut self, p: P) -> PResult<bool> {
        if self.is(p) {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn expect(&mut self, p: P) -> PResult<Token> {
        if self.is(p) {
            self.next()
        } else {
            self.unexpected()
        }
    }

    /// The current token is the identifier-like word `w`, written without escapes.
    pub fn is_word(&self, w: &str) -> bool {
        self.tok.kind == Tok::Ident && !self.tok.escaped && self.tok.ident == w
    }

    pub fn eat_word(&mut self, w: &str) -> PResult<bool> {
        if self.is_word(w) {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn expect_word(&mut self, w: &str) -> PResult<Token> {
        if self.is_word(w) {
            self.next()
        } else {
            self.unexpected()
        }
    }

    pub fn unexpected<T>(&self) -> PResult<T> {
        let what = match self.tok.kind {
            Tok::Eof => "end of input".to_string(),
            Tok::Ident | Tok::PrivateName => format!("'{}'", self.tok.ident),
            _ => format!("'{}'", &self.lx.source()[self.tok.start..self.tok.end]),
        };
        Err(SyntaxError::new(
            format!("unexpected token {what}"),
            self.tok.start,
        ))
    }

    pub fn err<T>(&self, msg: impl Into<String>, pos: usize) -> PResult<T> {
        Err(SyntaxError::new(msg, pos))
    }

    pub fn start(&self) -> usize {
        self.tok.start
    }

    pub fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.prev_end)
    }

    /// Automatic semicolon insertion (§12.10).
    pub fn semicolon(&mut self) -> PResult<()> {
        if self.eat(P::Semi)? {
            return Ok(());
        }
        if self.is(P::RBrace) || self.tok.kind == Tok::Eof || self.tok.nl_before {
            return Ok(());
        }
        self.unexpected()
    }

    /// Whether a `;` could be inserted before the current token.
    pub fn can_insert_semicolon(&self) -> bool {
        self.is(P::Semi) || self.is(P::RBrace) || self.tok.kind == Tok::Eof || self.tok.nl_before
    }

    // ---- context -------------------------------------------------------

    pub fn fn_ctx(&self) -> &FnCtx {
        self.fns.last().expect("function context")
    }

    pub fn fn_ctx_mut(&mut self) -> &mut FnCtx {
        self.fns.last_mut().expect("function context")
    }

    pub fn push_fn(&mut self, ctx: FnCtx) {
        self.fns.push(ctx);
    }

    pub fn pop_fn(&mut self) -> FnCtx {
        let ctx = self.fns.pop().expect("function context");
        // Arrow functions use their enclosing function's `this`/`arguments`,
        // and any direct eval can see the enclosing scopes.
        if let Some(outer) = self.fns.last_mut() {
            if ctx.is_arrow {
                outer.uses_this |= ctx.uses_this;
                outer.uses_arguments |= ctx.uses_arguments;
            }
            outer.has_direct_eval |= ctx.has_direct_eval;
        }
        ctx
    }

    /// `yield` is a keyword here (generator body).
    pub fn in_generator(&self) -> bool {
        self.fn_ctx().is_generator
    }

    /// `await` is a keyword here (async body, module, or class static block).
    pub fn in_async(&self) -> bool {
        self.fn_ctx().is_async || self.module
    }

    pub fn await_reserved(&self) -> bool {
        self.in_async() || self.fn_ctx().in_static_block
    }

    // ---- identifiers ---------------------------------------------------

    /// Checks that `name` may be used as an IdentifierReference or
    /// BindingIdentifier in the current context.
    pub fn check_identifier(&self, name: &str, pos: usize, binding: bool) -> PResult<()> {
        if name == "yield" && (self.strict || self.in_generator()) {
            return self.err("'yield' is not a valid identifier here", pos);
        }
        if name == "await" && (self.await_reserved() || self.fn_ctx().in_params && self.in_async())
        {
            return self.err("'await' is not a valid identifier here", pos);
        }
        if name == "await" {
            let seen = &self.fn_ctx().await_ident_pos;
            seen.set(seen.get().or(Some(pos)));
        }
        if name != "yield" && name != "await" && crate::RESERVED_WORDS.contains(&name) {
            return self.err(format!("unexpected reserved word '{name}'"), pos);
        }
        if self.strict && crate::STRICT_RESERVED_WORDS.contains(&name) {
            return self.err(format!("'{name}' is reserved in strict mode"), pos);
        }
        if binding && self.strict && (name == "eval" || name == "arguments") {
            return self.err(format!("cannot bind '{name}' in strict mode"), pos);
        }
        Ok(())
    }

    /// Parses a BindingIdentifier.
    pub fn binding_identifier(&mut self) -> PResult<Ident> {
        if self.tok.kind != Tok::Ident {
            return self.unexpected();
        }
        let t = self.next()?;
        self.check_identifier(&t.ident, t.start, true)?;
        Ok(Ident {
            span: t.span(),
            name: t.ident,
        })
    }

    /// Records a `#name` reference for resolution at the end of the class body.
    pub fn reference_private(&mut self, name: &str, pos: usize) -> PResult<()> {
        match self.classes.last_mut() {
            Some(scope) => {
                scope.unresolved.push((name.to_string(), pos));
                Ok(())
            }
            None if self.outer_private.iter().any(|n| n == name) => Ok(()),
            None => self.err(format!("undeclared private name #{name}"), pos),
        }
    }

    pub fn finish_private_names(&self) -> PResult<()> {
        Ok(())
    }

    // ---- statement lists with directives -----------------------------

    /// Parses a StatementList preceded by a directive prologue, until `end`.
    /// Sets `self.strict` when a "use strict" directive is present.
    pub fn parse_directives_and_statements(
        &mut self,
        allow_directives: bool,
        end: impl Fn(&Parser) -> bool,
    ) -> PResult<Vec<Stmt>> {
        let mut body = Vec::new();
        let mut saw_use_strict = false;
        let mut in_prologue = allow_directives;
        let mut octal_in_prologue: Option<usize> = None;
        while !end(self) {
            if in_prologue {
                if self.tok.kind == Tok::String {
                    let tok = self.tok.clone();
                    let stmt = self.parse_statement_list_item()?;
                    let is_directive = matches!(&stmt, Stmt::Expr(Expr::String(_, s), _) if s.start as usize == tok.start && s.end as usize == tok.end);
                    if is_directive {
                        if tok.legacy_octal && octal_in_prologue.is_none() {
                            octal_in_prologue = Some(tok.start);
                        }
                        let raw = &self.lx.source()[tok.start + 1..tok.end - 1];
                        if raw == "use strict" {
                            saw_use_strict = true;
                            self.strict = true;
                            if let Some(pos) = octal_in_prologue {
                                return self.err(
                                    "octal escape sequences are not allowed in strict mode",
                                    pos,
                                );
                            }
                        }
                    } else {
                        in_prologue = false;
                    }
                    body.push(stmt);
                    continue;
                }
                in_prologue = false;
            }
            body.push(self.parse_statement_list_item()?);
        }
        self.saw_use_strict = saw_use_strict;
        Ok(body)
    }
}
