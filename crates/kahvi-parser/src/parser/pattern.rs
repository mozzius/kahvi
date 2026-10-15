//! Binding patterns, and reinterpreting cover-grammar expressions as
//! assignment patterns or arrow parameters (ECMA-262 §13.15.5, §15.3).

use super::{PResult, Parser};
use crate::ast::*;
use crate::lexer::{P, Tok};

impl Parser<'_> {
    /// BindingIdentifier or BindingPattern.
    pub fn parse_binding_target(&mut self) -> PResult<Pattern> {
        match self.tok.kind {
            Tok::Punct(P::LBrace) => self.parse_object_binding_pattern(),
            Tok::Punct(P::LBracket) => self.parse_array_binding_pattern(),
            _ => Ok(Pattern::Ident(self.binding_identifier()?)),
        }
    }

    /// BindingElement: a target with an optional initializer.
    pub fn parse_binding_element(&mut self) -> PResult<PatternElem> {
        let target = self.parse_binding_target()?;
        let default = if self.eat(P::Eq)? {
            Some(self.with_in(true, |p| p.parse_assignment())?)
        } else {
            None
        };
        Ok(PatternElem { target, default })
    }

    fn parse_object_binding_pattern(&mut self) -> PResult<Pattern> {
        let start = self.start();
        self.next()?;
        let mut props = Vec::new();
        let mut rest = None;
        while !self.is(P::RBrace) {
            if self.eat(P::Ellipsis)? {
                rest = Some(Box::new(Pattern::Ident(self.binding_identifier()?)));
                break;
            }
            let pstart = self.start();
            let key_tok = self.tok.clone();
            let key = self.parse_property_key()?;
            if let PropKey::Private(_, span) = &key {
                return self.err("unexpected private name", span.start as usize);
            }
            if self.eat(P::Colon)? {
                let value = self.parse_binding_element()?;
                props.push(ObjectPatternProp {
                    key,
                    value,
                    shorthand: false,
                    span: self.span_from(pstart),
                });
            } else {
                let PropKey::Ident(name, span) = &key else {
                    return self.unexpected();
                };
                if key_tok.kind != Tok::Ident {
                    return self.unexpected();
                }
                self.check_identifier(name, span.start as usize, true)?;
                if key_tok.escaped && crate::RESERVED_WORDS.contains(&name.as_str()) {
                    return self.err(
                        "keywords cannot contain escape characters",
                        span.start as usize,
                    );
                }
                let target = Pattern::Ident(Ident {
                    name: name.clone(),
                    span: *span,
                });
                let default = if self.eat(P::Eq)? {
                    Some(self.with_in(true, |p| p.parse_assignment())?)
                } else {
                    None
                };
                props.push(ObjectPatternProp {
                    key,
                    value: PatternElem { target, default },
                    shorthand: true,
                    span: self.span_from(pstart),
                });
            }
            if !self.is(P::RBrace) {
                self.expect(P::Comma)?;
            }
        }
        self.expect(P::RBrace)?;
        Ok(Pattern::Object {
            props,
            rest,
            span: self.span_from(start),
        })
    }

    fn parse_array_binding_pattern(&mut self) -> PResult<Pattern> {
        let start = self.start();
        self.next()?;
        let mut elements = Vec::new();
        let mut rest = None;
        while !self.is(P::RBracket) {
            if self.eat(P::Comma)? {
                elements.push(None);
                continue;
            }
            if self.eat(P::Ellipsis)? {
                rest = Some(Box::new(self.parse_binding_target()?));
                break;
            }
            elements.push(Some(self.parse_binding_element()?));
            if !self.is(P::RBracket) {
                self.expect(P::Comma)?;
            }
        }
        self.expect(P::RBracket)?;
        Ok(Pattern::Array {
            elements,
            rest,
            span: self.span_from(start),
        })
    }

    /// Reinterprets an expression as an assignment target (`=`, for-in/of
    /// heads). Object and array literals become patterns if `allow_pattern`.
    pub fn expr_to_assignment_target(
        &mut self,
        expr: Expr,
        allow_pattern: bool,
    ) -> PResult<Pattern> {
        match expr {
            Expr::Ident(id) => {
                if self.strict && (id.name == "eval" || id.name == "arguments") {
                    return self.err(
                        format!("cannot assign to '{}' in strict mode", id.name),
                        id.span.start as usize,
                    );
                }
                Ok(Pattern::Ident(id))
            }
            Expr::Member {
                optional: false, ..
            }
            | Expr::SuperMember(..) => Ok(Pattern::Expr(Box::new(expr))),
            Expr::Paren(inner, span) => match *inner {
                e @ (Expr::Ident(_)
                | Expr::Member {
                    optional: false, ..
                }
                | Expr::SuperMember(..)
                | Expr::Paren(..)) => self.expr_to_assignment_target(e, false),
                _ => self.err("invalid assignment target", span.start as usize),
            },
            Expr::Object(props, span) if allow_pattern => {
                self.object_to_pattern(props, span, false)
            }
            Expr::Array(elems, span) if allow_pattern => self.array_to_pattern(elems, span, false),
            other => self.err("invalid assignment target", other.span().start as usize),
        }
    }

    /// Reinterprets an arrow function parameter expression.
    pub fn expr_to_param(&mut self, expr: Expr) -> PResult<Param> {
        match expr {
            Expr::Assign {
                op: AssignOp::Assign,
                target,
                value,
                span,
            } => {
                // A parenthesized target (`((x) = y) => {}`) is not a BindingElement.
                if target.span().start != span.start {
                    return self.err("invalid destructuring target", span.start as usize);
                }
                let target = self.pattern_to_binding(*target)?;
                Ok(Param {
                    target,
                    default: Some(*value),
                })
            }
            e => Ok(Param {
                target: self.expr_to_binding_pattern(e)?,
                default: None,
            }),
        }
    }

    pub fn expr_to_binding_pattern(&mut self, expr: Expr) -> PResult<Pattern> {
        match expr {
            Expr::Ident(id) => {
                self.check_identifier(&id.name, id.span.start as usize, true)?;
                Ok(Pattern::Ident(id))
            }
            Expr::Object(props, span) => self.object_to_pattern(props, span, true),
            Expr::Array(elems, span) => self.array_to_pattern(elems, span, true),
            other => self.err("invalid destructuring target", other.span().start as usize),
        }
    }

    /// Checks that an assignment pattern is also a valid binding pattern.
    pub fn pattern_to_binding(&mut self, pattern: Pattern) -> PResult<Pattern> {
        match pattern {
            Pattern::Ident(id) => {
                self.check_identifier(&id.name, id.span.start as usize, true)?;
                Ok(Pattern::Ident(id))
            }
            Pattern::Expr(e) => self.err("invalid destructuring target", e.span().start as usize),
            Pattern::Object { props, rest, span } => {
                let mut out = Vec::new();
                for p in props {
                    let target = self.pattern_to_binding(p.value.target)?;
                    out.push(ObjectPatternProp {
                        value: PatternElem {
                            target,
                            default: p.value.default,
                        },
                        ..p
                    });
                }
                let rest = match rest {
                    Some(r) => Some(Box::new(self.pattern_to_binding(*r)?)),
                    None => None,
                };
                Ok(Pattern::Object {
                    props: out,
                    rest,
                    span,
                })
            }
            Pattern::Array {
                elements,
                rest,
                span,
            } => {
                let mut out = Vec::new();
                for e in elements {
                    out.push(match e {
                        Some(e) => Some(PatternElem {
                            target: self.pattern_to_binding(e.target)?,
                            default: e.default,
                        }),
                        None => None,
                    });
                }
                let rest = match rest {
                    Some(r) => Some(Box::new(self.pattern_to_binding(*r)?)),
                    None => None,
                };
                Ok(Pattern::Array {
                    elements: out,
                    rest,
                    span,
                })
            }
        }
    }

    fn element_target(&mut self, expr: Expr, binding: bool) -> PResult<PatternElem> {
        match expr {
            Expr::Assign {
                op: AssignOp::Assign,
                target,
                value,
                ..
            } => {
                let target = if binding {
                    self.pattern_to_binding(*target)?
                } else {
                    *target
                };
                Ok(PatternElem {
                    target,
                    default: Some(*value),
                })
            }
            e => {
                let target = if binding {
                    self.expr_to_binding_pattern(e)?
                } else {
                    self.expr_to_assignment_target(e, true)?
                };
                Ok(PatternElem {
                    target,
                    default: None,
                })
            }
        }
    }

    fn object_to_pattern(
        &mut self,
        props: Vec<ObjectProp>,
        span: Span,
        binding: bool,
    ) -> PResult<Pattern> {
        if self.spread_trailing_comma.contains(&(span.start as usize)) {
            return self.err(
                "rest element may not have a trailing comma",
                span.start as usize,
            );
        }
        let mut out = Vec::new();
        let mut rest = None;
        let n = props.len();
        for (i, prop) in props.into_iter().enumerate() {
            match prop {
                ObjectProp::Init(key, value, pspan) => {
                    let value = self.element_target(value, binding)?;
                    out.push(ObjectPatternProp {
                        key,
                        value,
                        shorthand: false,
                        span: pspan,
                    });
                }
                ObjectProp::Proto(value, pspan) => {
                    let value = self.element_target(value, binding)?;
                    let key = PropKey::Ident("__proto__".into(), pspan);
                    out.push(ObjectPatternProp {
                        key,
                        value,
                        shorthand: false,
                        span: pspan,
                    });
                }
                ObjectProp::Shorthand(id) => {
                    let target = if binding {
                        self.expr_to_binding_pattern(Expr::Ident(id.clone()))?
                    } else {
                        self.expr_to_assignment_target(Expr::Ident(id.clone()), false)?
                    };
                    out.push(ObjectPatternProp {
                        key: PropKey::Ident(id.name.clone(), id.span),
                        value: PatternElem {
                            target,
                            default: None,
                        },
                        shorthand: true,
                        span: id.span,
                    });
                }
                ObjectProp::CoverInit(id, default, pspan) => {
                    let target = if binding {
                        self.expr_to_binding_pattern(Expr::Ident(id.clone()))?
                    } else {
                        self.expr_to_assignment_target(Expr::Ident(id.clone()), false)?
                    };
                    out.push(ObjectPatternProp {
                        key: PropKey::Ident(id.name.clone(), id.span),
                        value: PatternElem {
                            target,
                            default: Some(default),
                        },
                        shorthand: true,
                        span: pspan,
                    });
                }
                ObjectProp::Spread(e, pspan) => {
                    if i != n - 1 {
                        return self.err("rest element must be last", pspan.start as usize);
                    }
                    // `...` in an object pattern takes a simple target, not a pattern.
                    let target = if binding {
                        match e {
                            Expr::Ident(id) => {
                                self.check_identifier(&id.name, id.span.start as usize, true)?;
                                Pattern::Ident(id)
                            }
                            other => {
                                return self
                                    .err("invalid rest element", other.span().start as usize);
                            }
                        }
                    } else {
                        self.expr_to_assignment_target(e, false)?
                    };
                    rest = Some(Box::new(target));
                }
                ObjectProp::Method(_, f) | ObjectProp::Getter(_, f) | ObjectProp::Setter(_, f) => {
                    return self.err("invalid destructuring target", f.span.start as usize);
                }
            }
        }
        Ok(Pattern::Object {
            props: out,
            rest,
            span,
        })
    }

    fn array_to_pattern(
        &mut self,
        elems: Vec<Option<ArrayElem>>,
        span: Span,
        binding: bool,
    ) -> PResult<Pattern> {
        if self.spread_trailing_comma.contains(&(span.start as usize)) {
            return self.err(
                "rest element may not have a trailing comma",
                span.start as usize,
            );
        }
        let mut out = Vec::new();
        let mut rest = None;
        let n = elems.len();
        for (i, elem) in elems.into_iter().enumerate() {
            match elem {
                None => out.push(None),
                Some(ArrayElem::Expr(e)) => out.push(Some(self.element_target(e, binding)?)),
                Some(ArrayElem::Spread(e)) => {
                    if i != n - 1 {
                        return self.err("rest element must be last", e.span().start as usize);
                    }
                    if let Expr::Assign { span, .. } = &e {
                        return self
                            .err("rest element may not have a default", span.start as usize);
                    }
                    let target = if binding {
                        self.expr_to_binding_pattern(e)?
                    } else {
                        self.expr_to_assignment_target(e, true)?
                    };
                    rest = Some(Box::new(target));
                }
            }
        }
        Ok(Pattern::Array {
            elements: out,
            rest,
            span,
        })
    }
}
