//! Class definitions (ClassDefinitionEvaluation, §15.7.14).
//!
//! Layout of the emitted code:
//!
//! ```text
//! [name]                      the class name (NamedEvaluation or its own)
//! PushScope(classEnv)         with an immutable binding for the class name
//! <heritage>?
//! ClassHeritage               [name proto ctorParent]
//! PushPrivateScope(class)
//! CreateClass                 [F proto]
//! <elements>                  methods defined now, fields recorded on F
//! Pop                         [F]
//! Dup InitName(name)          classEnv.InitializeBinding(name, F)
//! PopScope PopPrivateScope
//! FinishClass                 static fields and blocks run
//! ```

use kahvi_parser::ast::{Class, ClassElement, MethodKind as AstMethodKind, PropKey};

use super::expr::FnName;
use super::{
    ClassInfo, ClassStatic, CodeKind, Compiler, FieldDef, FunctionFlavor, JumpScope, MethodDef,
    MethodKind, Op, PrivateMethodDef, ScopeInfo,
};
use crate::function::ThisMode;

impl Compiler<'_> {
    pub(super) fn compile_class(&mut self, c: &Class, name: FnName) {
        let own_name = c.id.as_ref().map(|i| self.agent.string(&i.name));
        // The name F gets: its own binding name, the NamedEvaluation name, or "".
        match name {
            FnName::OnStack => {}
            FnName::Static(s) => {
                self.emit(Op::String(s));
            }
            FnName::Own => {
                let s = own_name.unwrap_or(self.agent.names.empty);
                self.emit(Op::String(s));
            }
        }

        // classEnv, with the class binding (immutable) if the class has a name.
        let lexical = own_name.map(|n| vec![(n, true)]).unwrap_or_default();
        self.code.scopes.push(ScopeInfo {
            lexical,
            functions: Vec::new(),
            is_catch: false,
        });
        let scope = (self.code.scopes.len() - 1) as u32;
        self.emit(Op::PushScope(scope));
        self.scopes.push(JumpScope::Env);
        // All parts of a class are strict mode code (sec-strict-mode-code).
        let strict_start = self.code.ops.len() as u32;

        // ClassHeritage, evaluated in classEnv with the outer private environment.
        if let Some(h) = &c.super_class {
            self.compile_expr(h);
        }
        self.position(c.span);
        self.emit(Op::ClassHeritage(c.super_class.is_some()));

        // The class's private names.
        let mut private_names = Vec::new();
        for e in &c.elements {
            let key = match e {
                ClassElement::Method { key, .. }
                | ClassElement::Field { key, .. }
                | ClassElement::Accessor { key, .. } => key,
                ClassElement::StaticBlock(_) => continue,
            };
            if let PropKey::Private(n, _) = key {
                let s = self.string(&format!("#{n}"));
                if !private_names.contains(&s) {
                    private_names.push(s);
                }
            }
        }
        self.code.classes.push(ClassStatic {
            name: own_name,
            private_names,
            span: (c.span.start, c.span.end),
        });
        let class_index = (self.code.classes.len() - 1) as u32;
        self.emit(Op::PushPrivateScope(class_index));
        self.scopes.push(JumpScope::PrivateScope);

        // The constructor.
        let constructor = match &c.constructor {
            Some(f) => self.compile_nested_function(f, false),
            None => self.default_constructor(c),
        };
        self.emit(Op::CreateClass(ClassInfo {
            constructor: Some(constructor),
            class: class_index,
        }));

        // [F proto]
        for e in &c.elements {
            self.compile_class_element(e);
        }
        self.emit(Op::Pop);
        if let Some(n) = own_name {
            self.emit(Op::Dup);
            self.emit(Op::InitName(n));
        }
        self.scopes.pop();
        self.emit(Op::PopPrivateScope);
        self.scopes.pop();
        self.emit(Op::PopScope);
        self.emit(Op::FinishClass);
        if !self.code.strict {
            let end = self.code.ops.len() as u32;
            self.code.strict_ranges.push((strict_start, end));
        }
    }

    /// A constructor for a class without one: `constructor() {}` or
    /// `constructor(...args) { super(...args) }` (without iterating args).
    fn default_constructor(&mut self, c: &Class) -> u32 {
        let source = self.code.source.clone();
        let script = self.code.script;
        let mut d = Compiler::new(self.agent, CodeKind::Function, source, script, c.span);
        d.code.strict = true;
        d.code.this_mode = ThisMode::Strict;
        d.code.params.simple = true;
        let derived = c.super_class.is_some();
        d.code.flavor = if derived {
            FunctionFlavor::DerivedConstructor
        } else {
            FunctionFlavor::ClassConstructor
        };
        d.emit(Op::InstantiateFunctionBody);
        if derived {
            d.emit(Op::SuperCallForward);
            d.emit(Op::Pop);
        }
        d.emit(Op::Undefined);
        d.emit(Op::Return);
        let code = d.finish();
        self.add_function(code)
    }

    /// One class element, with [F proto] on the stack (left unchanged).
    fn compile_class_element(&mut self, e: &ClassElement) {
        match e {
            ClassElement::Method {
                key,
                kind,
                is_static,
                function,
            } => {
                let kind = match kind {
                    AstMethodKind::Method => MethodKind::Method,
                    AstMethodKind::Getter => MethodKind::Getter,
                    AstMethodKind::Setter => MethodKind::Setter,
                };
                let index = self.compile_nested_function(function, false);
                if let PropKey::Private(n, _) = key {
                    let name = self.string(&format!("#{n}"));
                    self.emit(Op::Closure(index));
                    self.emit(Op::DefinePrivateMethod(PrivateMethodDef {
                        name,
                        kind,
                        is_static: *is_static,
                    }));
                    return;
                }
                // [F proto target]
                self.emit(if *is_static { Op::Pick(1) } else { Op::Dup });
                self.compile_prop_key(key);
                self.emit(Op::Closure(index));
                self.emit(Op::DefineMethod(MethodDef {
                    kind,
                    enumerable: false,
                }));
                self.emit(Op::Pop);
            }
            ClassElement::Field {
                key,
                is_static,
                init,
                ..
            } => {
                self.compile_prop_key(key);
                match init {
                    Some(f) => {
                        let index = self.compile_nested_function(f, false);
                        self.emit(Op::Closure(index));
                    }
                    None => {
                        self.emit(Op::Undefined);
                    }
                }
                let private = matches!(key, PropKey::Private(..));
                self.emit(Op::DefineClassField(FieldDef {
                    is_static: *is_static,
                    private,
                    static_block: false,
                    auto_accessor: false,
                }));
            }
            ClassElement::Accessor { .. } => self.not_implemented("class auto-accessors"),
            ClassElement::StaticBlock(f) => {
                let index = self.compile_nested_function(f, false);
                self.emit(Op::Closure(index));
                self.emit(Op::DefineClassField(FieldDef {
                    is_static: true,
                    private: false,
                    static_block: true,
                    auto_accessor: false,
                }));
            }
        }
    }
}
