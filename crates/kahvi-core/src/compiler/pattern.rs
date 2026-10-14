//! Binding initialization (declarations, parameters) and destructuring
//! assignment. Both take the value on top of the stack and consume it.

use kahvi_parser::ast::{Expr, MemberProp, ObjectPatternProp, Pattern, PatternElem, PropKey};

use super::{Compiler, ErrorType, JumpScope, Op};

/// How an identifier in a pattern is bound.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// InitializeReferencedBinding (let/const/class/parameters without duplicates).
    Init,
    /// PutValue through ResolveBinding (var, sloppy duplicate parameters).
    Put,
    /// Destructuring assignment: targets may be any reference.
    Assign,
}

impl Compiler<'_> {
    /// BindingInitialization: [value] -> []. `init` selects
    /// InitializeReferencedBinding over PutValue.
    pub(super) fn compile_binding(&mut self, target: &Pattern, init: bool) {
        self.bind(target, if init { Mode::Init } else { Mode::Put });
    }

    /// DestructuringAssignmentEvaluation: [value] -> [].
    pub(super) fn compile_assignment_pattern(&mut self, target: &Pattern) {
        self.bind(target, Mode::Assign);
    }

    fn bind(&mut self, target: &Pattern, mode: Mode) {
        match target {
            Pattern::Ident(id) => {
                let name = self.string(&id.name);
                if mode == Mode::Init {
                    self.emit(Op::InitName(name));
                } else {
                    self.emit(Op::ResolveName(name));
                    self.emit(Op::PutRef);
                    self.emit(Op::Pop);
                }
            }
            Pattern::Expr(e) => {
                // Only in assignment patterns: evaluate the reference after the value.
                self.assign_to_expr_with_value(e);
            }
            Pattern::Object { props, rest, span } => {
                self.position(*span);
                self.emit(Op::RequireObjectCoercible);
                self.bind_object(props, rest.as_deref(), mode);
            }
            Pattern::Array {
                elements,
                rest,
                span,
            } => {
                self.position(*span);
                self.bind_array(elements, rest.as_deref(), mode);
            }
        }
    }

    /// [value] -> [] for an element target that needs its reference
    /// evaluated (member expressions in assignment patterns).
    fn assign_to_expr_with_value(&mut self, e: &Expr) {
        match e.unparen() {
            Expr::Member {
                object, property, ..
            } => {
                self.compile_expr(object);
                match property {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::Swap);
                        self.emit(Op::SetProp(k));
                    }
                    MemberProp::Computed(k) => {
                        self.compile_expr(k);
                        // [value base key] -> [base key value]
                        self.emit(Op::RotRight3);
                        self.emit(Op::RotRight3);
                        self.emit(Op::SetElem);
                    }
                    MemberProp::Private(name, _) => {
                        let s = self.string(&format!("#{name}"));
                        self.emit(Op::Swap);
                        self.emit(Op::SetPrivate(s));
                    }
                }
                self.emit(Op::Pop);
            }
            Expr::SuperMember(prop @ (MemberProp::Ident(..) | MemberProp::Computed(_)), _) => {
                // [value] -> [value this key base] -> [value this key base value] -> PutValue.
                match prop {
                    MemberProp::Ident(name, _) => self.compile_super_reference_named(name),
                    MemberProp::Computed(k) => self.compile_super_reference(k),
                    MemberProp::Private(..) => unreachable!(),
                }
                self.emit(Op::Pick(3));
                self.emit(Op::SetSuperElem);
                self.emit(Op::Pop);
                self.emit(Op::Pop);
            }
            _ => {
                self.emit(Op::Pop);
                self.throw_error(ErrorType::Reference, "invalid destructuring target");
            }
        }
    }

    /// Whether a target is a simple member reference in an assignment pattern
    /// (its reference is evaluated before the value is fetched).
    fn is_member_target(target: &Pattern) -> bool {
        matches!(target, Pattern::Expr(e) if matches!(e.unparen(), Expr::Member { .. }))
    }

    /// Applies an element's default: [value] -> [value or default].
    fn apply_default(&mut self, elem: &PatternElem) {
        if let Some(d) = &elem.default {
            let j = self.emit(Op::JumpIfNotUndefined(0));
            self.emit(Op::Pop);
            self.compile_named_expr(d, &elem.target);
            self.patch(j);
        }
    }

    /// Binds one element whose value is produced by `fetch` (which pushes
    /// it). For member targets in assignment patterns the reference is
    /// evaluated first, as the spec requires.
    fn bind_element(
        &mut self,
        elem: &PatternElem,
        mode: Mode,
        fetch: &mut dyn FnMut(&mut Self, u32),
    ) {
        if mode == Mode::Assign && Self::is_member_target(&elem.target) {
            let Pattern::Expr(e) = &elem.target else {
                unreachable!()
            };
            let Expr::Member {
                object, property, ..
            } = e.unparen()
            else {
                unreachable!()
            };
            self.compile_expr(object);
            match property {
                MemberProp::Ident(name, _) => {
                    let k = self.agent.key(name);
                    fetch(self, 1);
                    self.apply_default(elem);
                    self.emit(Op::SetProp(k));
                }
                MemberProp::Computed(k) => {
                    self.compile_expr(k);
                    fetch(self, 2);
                    self.apply_default(elem);
                    self.emit(Op::SetElem);
                }
                MemberProp::Private(name, _) => {
                    let s = self.string(&format!("#{name}"));
                    fetch(self, 1);
                    self.apply_default(elem);
                    self.emit(Op::SetPrivate(s));
                }
            }
            self.emit(Op::Pop);
            return;
        }
        if mode != Mode::Init
            && let Pattern::Ident(id) = &elem.target
        {
            // ResolveBinding happens before the value is fetched.
            let name = self.string(&id.name);
            self.emit(Op::ResolveName(name));
            fetch(self, 0);
            self.apply_default(elem);
            self.emit(Op::PutRef);
            self.emit(Op::Pop);
            return;
        }
        fetch(self, 0);
        self.apply_default(elem);
        self.bind(&elem.target, mode);
    }

    /// [object] -> []
    fn bind_object(&mut self, props: &[ObjectPatternProp], rest: Option<&Pattern>, mode: Mode) {
        // With a rest element the keys are kept on the stack above the
        // object so they can be excluded: [object k1 .. kn].
        let keep_keys = rest.is_some();
        let mut kept = 0u32;
        for prop in props {
            match &prop.key {
                PropKey::Computed(_) => {
                    self.compile_prop_key(&prop.key);
                    if keep_keys {
                        self.emit(Op::Dup);
                    }
                    // [object (keys..) key]: fetch object[key] with the key consumed.
                    let depth = kept + if keep_keys { 1 } else { 0 };
                    self.bind_element(&prop.value, mode, &mut |c: &mut Self, extra: u32| {
                        // Stack: [object (keys..) key (target refs: extra)]
                        c.emit(Op::Pick(depth + 1 + extra));
                        c.emit(Op::Pick(1 + extra));
                        c.emit(Op::GetElem);
                    });
                    // Drop the key used for the fetch: [object (keys..) key]
                    self.emit(Op::Pop);
                    if keep_keys {
                        kept += 1;
                    }
                }
                _ => {
                    let k = self.static_key(&prop.key).expect("static key");
                    if keep_keys {
                        self.compile_prop_key(&prop.key);
                        kept += 1;
                    }
                    let depth = kept;
                    self.bind_element(&prop.value, mode, &mut |c: &mut Self, extra: u32| {
                        c.emit(Op::Pick(depth + extra));
                        c.emit(Op::GetProp(k));
                    });
                }
            }
        }
        match rest {
            Some(r) => {
                // [object k1..kn] -> [object rest]
                self.emit(Op::NewObject);
                self.emit(Op::Pick(kept + 1));
                self.emit(Op::CopyDataPropertiesExcluding(kept));
                // [object rest]
                self.bind(r, mode);
                self.emit(Op::Pop);
            }
            None => {
                self.emit(Op::Pop);
            }
        }
    }

    /// [iterable] -> []
    fn bind_array(&mut self, elements: &[Option<PatternElem>], rest: Option<&Pattern>, mode: Mode) {
        self.emit(Op::GetIterator);
        let handler = self.emit(Op::PushHandler(0));
        self.scopes.push(JumpScope::Iterator {
            close: true,
            is_async: false,
        });
        self.scopes.push(JumpScope::Handler);
        for el in elements {
            match el {
                None => {
                    self.emit(Op::IteratorNextOrUndefined);
                    self.emit(Op::Pop);
                }
                Some(elem) => {
                    self.bind_element(elem, mode, &mut |c: &mut Self, _| {
                        c.emit(Op::IteratorNextOrUndefined);
                    });
                }
            }
        }
        if let Some(r) = rest {
            if mode == Mode::Assign && Self::is_member_target(r) {
                let elem = PatternElem {
                    target: r.clone(),
                    default: None,
                };
                self.bind_element(&elem, mode, &mut |c: &mut Self, _| {
                    c.emit(Op::IteratorRest);
                });
            } else if mode != Mode::Init && matches!(r, Pattern::Ident(_)) {
                let elem = PatternElem {
                    target: r.clone(),
                    default: None,
                };
                self.bind_element(&elem, mode, &mut |c: &mut Self, _| {
                    c.emit(Op::IteratorRest);
                });
            } else {
                self.emit(Op::IteratorRest);
                self.bind(r, mode);
            }
        }
        self.scopes.pop();
        self.scopes.pop();
        self.emit(Op::PopHandler);
        // If the iterator is not done, close it.
        self.emit(Op::IteratorClose);
        let end = self.emit(Op::Jump(0));
        self.patch(handler);
        self.emit(Op::IteratorCloseForThrow);
        self.emit(Op::Throw);
        self.patch(end);
    }
}
