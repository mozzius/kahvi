//! Static semantics (BoundNames, VarDeclaredNames, LexicallyDeclaredNames…)
//! and the declaration-conflict early errors built on them. The compiler
//! uses the same functions to create bindings.

use crate::SyntaxError;
use crate::ast::*;

type R = Result<(), SyntaxError>;

fn dup_error(name: &str, span: Span) -> SyntaxError {
    SyntaxError::new(
        format!("identifier '{name}' has already been declared"),
        span.start as usize,
    )
}

/// BoundNames of a binding pattern.
pub fn bound_names(p: &Pattern) -> Vec<&str> {
    let mut out = Vec::new();
    collect_bound(p, &mut out);
    out
}

fn collect_bound<'a>(p: &'a Pattern, out: &mut Vec<&'a str>) {
    match p {
        Pattern::Ident(id) => out.push(&id.name),
        Pattern::Expr(_) => {}
        Pattern::Object { props, rest, .. } => {
            for prop in props {
                collect_bound(&prop.value.target, out);
            }
            if let Some(r) = rest {
                collect_bound(r, out);
            }
        }
        Pattern::Array { elements, rest, .. } => {
            for e in elements.iter().flatten() {
                collect_bound(&e.target, out);
            }
            if let Some(r) = rest {
                collect_bound(r, out);
            }
        }
    }
}

/// A declared name and where it was declared.
#[derive(Debug, Clone, Copy)]
pub struct Decl<'a> {
    pub name: &'a str,
    pub span: Span,
    /// Declared by a function declaration (matters for Annex B duplicates).
    pub is_function: bool,
}

/// VarDeclaredNames of a statement list (not descending into functions).
/// Function declarations are not included; see [`top_level_functions`].
pub fn var_declared_names(stmts: &[Stmt]) -> Vec<Decl<'_>> {
    let mut out = Vec::new();
    for s in stmts {
        var_names_stmt(s, &mut out);
    }
    out
}

fn push_pattern<'a>(p: &'a Pattern, out: &mut Vec<Decl<'a>>) {
    let span = p.span();
    for name in bound_names(p) {
        out.push(Decl {
            name,
            span,
            is_function: false,
        });
    }
}

fn var_names_stmt<'a>(s: &'a Stmt, out: &mut Vec<Decl<'a>>) {
    match s {
        Stmt::Var(d) if d.kind == VarKind::Var => {
            for decl in &d.declarations {
                push_pattern(&decl.target, out);
            }
        }
        Stmt::If {
            consequent,
            alternate,
            ..
        } => {
            var_names_stmt(consequent, out);
            if let Some(a) = alternate {
                var_names_stmt(a, out);
            }
        }
        Stmt::For { init, body, .. } => {
            if let Some(ForInit::Var(d)) = init
                && d.kind == VarKind::Var
            {
                for decl in &d.declarations {
                    push_pattern(&decl.target, out);
                }
            }
            var_names_stmt(body, out);
        }
        Stmt::ForIn { left, body, .. } | Stmt::ForOf { left, body, .. } => {
            if let ForHead::Var {
                kind: VarKind::Var,
                target,
                ..
            } = left
            {
                push_pattern(target, out);
            }
            var_names_stmt(body, out);
        }
        Stmt::While { body, .. } | Stmt::DoWhile { body, .. } | Stmt::With { body, .. } => {
            var_names_stmt(body, out)
        }
        Stmt::Labeled { body, .. } => var_names_stmt(body, out),
        Stmt::Block(b) => {
            for s in &b.body {
                var_names_stmt(s, out);
            }
        }
        Stmt::Switch { cases, .. } => {
            for c in cases {
                for s in &c.body {
                    var_names_stmt(s, out);
                }
            }
        }
        Stmt::Try {
            block,
            handler,
            finalizer,
            ..
        } => {
            for s in &block.body {
                var_names_stmt(s, out);
            }
            if let Some(h) = handler {
                for s in &h.body.body {
                    var_names_stmt(s, out);
                }
            }
            if let Some(f) = finalizer {
                for s in &f.body {
                    var_names_stmt(s, out);
                }
            }
        }
        _ => {}
    }
}

/// Function declarations directly in a function/script body (including
/// labelled ones): TopLevelVarScopedDeclarations' functions.
pub fn top_level_functions(stmts: &[Stmt]) -> Vec<&std::rc::Rc<Function>> {
    let mut out = Vec::new();
    for s in stmts {
        let mut s = s;
        while let Stmt::Labeled { body, .. } = s {
            s = body;
        }
        if let Stmt::Function(f) = s {
            out.push(f);
        }
    }
    out
}

/// LexicallyDeclaredNames of a statement list. At the top level of a
/// function or script, function declarations are var-scoped and excluded.
pub fn lexically_declared_names(stmts: &[Stmt], top_level: bool) -> Vec<Decl<'_>> {
    let mut out = Vec::new();
    for s in stmts {
        match s {
            Stmt::Var(d) if d.kind.is_lexical() => {
                for decl in &d.declarations {
                    push_pattern(&decl.target, &mut out);
                }
            }
            Stmt::Class(c) => {
                if let Some(id) = &c.id {
                    out.push(Decl {
                        name: &id.name,
                        span: id.span,
                        is_function: false,
                    });
                }
            }
            Stmt::Function(f) if !top_level => {
                if let Some(id) = &f.id {
                    out.push(Decl {
                        name: &id.name,
                        span: id.span,
                        is_function: is_plain_function(f),
                    });
                }
            }
            Stmt::Labeled { .. } if !top_level => {
                let mut inner = s;
                while let Stmt::Labeled { body, .. } = inner {
                    inner = body;
                }
                if let Stmt::Function(f) = inner
                    && let Some(id) = &f.id
                {
                    out.push(Decl {
                        name: &id.name,
                        span: id.span,
                        is_function: is_plain_function(f),
                    });
                }
            }
            _ => {}
        }
    }
    out
}

fn check_duplicates(decls: &[Decl], allow_function_duplicates: bool) -> R {
    for (i, d) in decls.iter().enumerate() {
        if let Some(prev) = decls[..i].iter().find(|p| p.name == d.name) {
            if allow_function_duplicates && prev.is_function && d.is_function {
                continue;
            }
            return Err(dup_error(d.name, d.span));
        }
    }
    Ok(())
}

fn check_lexical_vs_var(lexical: &[Decl], vars: &[Decl]) -> R {
    for v in vars {
        if let Some(l) = lexical.iter().find(|l| l.name == v.name) {
            let span = if l.span.start > v.span.start {
                l.span
            } else {
                v.span
            };
            return Err(dup_error(v.name, span));
        }
    }
    Ok(())
}

/// Annex B.3.2.4 only tolerates duplicates among plain function declarations.
fn is_plain_function(f: &Function) -> bool {
    !f.is_async && !f.is_generator
}

/// Block / switch case block: no duplicate lexical names, no var colliding
/// with them. (Annex B: duplicate plain function declarations are allowed in
/// sloppy code.)
pub fn check_block_declarations(body: &[Stmt], strict: bool) -> R {
    let lexical = lexically_declared_names(body, false);
    check_duplicates(&lexical, !strict)?;
    check_lexical_vs_var(&lexical, &var_declared_names(body))
}

/// Function bodies: lexical names must not repeat, collide with vars,
/// or collide with parameters.
pub fn check_function_body(params: &[(&str, usize)], body: &[Stmt]) -> R {
    let lexical = lexically_declared_names(body, true);
    check_duplicates(&lexical, false)?;
    let mut vars = var_declared_names(body);
    for f in top_level_functions(body) {
        if let Some(id) = &f.id {
            vars.push(Decl {
                name: &id.name,
                span: id.span,
                is_function: true,
            });
        }
    }
    check_lexical_vs_var(&lexical, &vars)?;
    for (name, _) in params {
        if let Some(l) = lexical.iter().find(|l| l.name == *name) {
            return Err(dup_error(name, l.span));
        }
    }
    Ok(())
}

/// Scripts: as function bodies, without parameters.
pub fn check_script_declarations(body: &[Stmt], _eval_like: bool) -> R {
    check_function_body(&[], body)
}

/// `catch (param) { ... }`
pub fn check_catch(param: &Pattern, body: &[Stmt]) -> R {
    let names = bound_names(param);
    for (i, n) in names.iter().enumerate() {
        if names[..i].contains(n) {
            return Err(dup_error(n, param.span()));
        }
    }
    let lexical = lexically_declared_names(body, false);
    for l in &lexical {
        if names.contains(&l.name) {
            return Err(dup_error(l.name, l.span));
        }
    }
    // Annex B.3.4: `var` may redeclare a simple catch parameter, except in for-of.
    let simple = matches!(param, Pattern::Ident(_));
    let vars = if simple {
        for_of_var_names(body)
    } else {
        var_declared_names(body)
    };
    for v in &vars {
        if names.contains(&v.name) {
            return Err(dup_error(v.name, v.span));
        }
    }
    Ok(())
}

fn for_of_var_names(stmts: &[Stmt]) -> Vec<Decl<'_>> {
    let mut out = Vec::new();
    fn walk<'a>(s: &'a Stmt, out: &mut Vec<Decl<'a>>) {
        match s {
            Stmt::ForOf {
                left:
                    ForHead::Var {
                        kind: VarKind::Var,
                        target,
                        ..
                    },
                body,
                ..
            } => {
                push_pattern(target, out);
                walk(body, out);
            }
            Stmt::Block(b) => b.body.iter().for_each(|s| walk(s, out)),
            Stmt::If {
                consequent,
                alternate,
                ..
            } => {
                walk(consequent, out);
                if let Some(a) = alternate {
                    walk(a, out);
                }
            }
            Stmt::For { body, .. }
            | Stmt::ForIn { body, .. }
            | Stmt::ForOf { body, .. }
            | Stmt::While { body, .. }
            | Stmt::DoWhile { body, .. }
            | Stmt::With { body, .. }
            | Stmt::Labeled { body, .. } => walk(body, out),
            Stmt::Try {
                block,
                handler,
                finalizer,
                ..
            } => {
                block.body.iter().for_each(|s| walk(s, out));
                if let Some(h) = handler {
                    h.body.body.iter().for_each(|s| walk(s, out));
                }
                if let Some(f) = finalizer {
                    f.body.iter().for_each(|s| walk(s, out));
                }
            }
            Stmt::Switch { cases, .. } => cases
                .iter()
                .flat_map(|c| &c.body)
                .for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    stmts.iter().for_each(|s| walk(s, &mut out));
    out
}

/// `for (let x; ;) { var x }` and duplicates in the head.
pub fn check_for_head_vs_body(decl: &VarDecl, body: &Stmt) -> R {
    let mut names = Vec::new();
    for d in &decl.declarations {
        push_pattern(&d.target, &mut names);
    }
    check_duplicates(&names, false)?;
    let mut vars = Vec::new();
    var_names_stmt(body, &mut vars);
    check_lexical_vs_var(&names, &vars)
}

/// `for (let [a, b] of ...) { var a }`
pub fn check_for_binding_vs_body(target: &Pattern, body: &Stmt) -> R {
    let mut names = Vec::new();
    push_pattern(target, &mut names);
    for n in &names {
        if n.name == "let" {
            return Err(SyntaxError::new(
                "'let' cannot be a lexically bound name",
                n.span.start as usize,
            ));
        }
    }
    check_duplicates(&names, false)?;
    let mut vars = Vec::new();
    var_names_stmt(body, &mut vars);
    check_lexical_vs_var(&names, &vars)
}

/// Module-level declaration and export checks (§16.2.1.1).
pub fn check_module(items: &[ModuleItem]) -> R {
    let mut stmts: Vec<Stmt> = Vec::new();
    let mut lexical_extra: Vec<(String, Span)> = Vec::new();
    let mut exported: Vec<(Vec<u16>, Span)> = Vec::new();
    let mut local_refs: Vec<(String, Span)> = Vec::new();
    for item in items {
        match item {
            ModuleItem::Stmt(s) => stmts.push(s.clone()),
            ModuleItem::Import(i) => {
                for id in i
                    .default
                    .iter()
                    .chain(i.namespace.iter())
                    .chain(i.named.iter().map(|s| &s.local))
                {
                    lexical_extra.push((id.name.clone(), i.span));
                }
            }
            ModuleItem::Export(e) => match e {
                ExportDecl::Named {
                    specifiers,
                    source,
                    span,
                    ..
                } => {
                    for s in specifiers {
                        exported.push((export_name_utf16(&s.exported), *span));
                        if source.is_none()
                            && let ModuleExportName::Ident(n) = &s.local
                        {
                            local_refs.push((n.clone(), *span));
                        }
                    }
                }
                ExportDecl::All {
                    alias: Some(a),
                    span,
                    ..
                } => exported.push((export_name_utf16(a), *span)),
                ExportDecl::All { .. } => {}
                ExportDecl::Decl(s) => {
                    for n in declared_names_of(s) {
                        exported.push((n.encode_utf16().collect(), s.span()));
                    }
                    stmts.push(s.clone());
                }
                ExportDecl::DefaultDecl(s) => {
                    exported.push(("default".encode_utf16().collect(), s.span()));
                    let has_name = match s {
                        Stmt::Function(f) => f.id.is_some(),
                        Stmt::Class(c) => c.id.is_some(),
                        _ => false,
                    };
                    if has_name {
                        stmts.push(s.clone());
                    } else {
                        lexical_extra.push(("*default*".into(), s.span()));
                    }
                }
                ExportDecl::DefaultExpr(_, span) => {
                    exported.push(("default".encode_utf16().collect(), *span));
                    lexical_extra.push(("*default*".into(), *span));
                }
            },
        }
    }
    // In modules, top-level function declarations are lexical.
    let mut lexical = lexically_declared_names(&stmts, false);
    for (name, span) in &lexical_extra {
        lexical.push(Decl {
            name,
            span: *span,
            is_function: false,
        });
    }
    check_duplicates(&lexical, false)?;
    check_lexical_vs_var(&lexical, &var_declared_names(&stmts))?;
    for (i, (name, span)) in exported.iter().enumerate() {
        if exported[..i].iter().any(|(n, _)| n == name) {
            return Err(SyntaxError::new(
                format!("duplicate export '{}'", String::from_utf16_lossy(name)),
                span.start as usize,
            ));
        }
    }
    let vars = var_declared_names(&stmts);
    for (name, span) in &local_refs {
        let declared =
            lexical.iter().any(|d| d.name == name) || vars.iter().any(|d| d.name == name);
        if !declared {
            return Err(SyntaxError::new(
                format!("export '{name}' is not defined"),
                span.start as usize,
            ));
        }
    }
    Ok(())
}

fn export_name_utf16(n: &ModuleExportName) -> Vec<u16> {
    match n {
        ModuleExportName::Ident(s) => s.encode_utf16().collect(),
        ModuleExportName::String(s) => s.clone(),
    }
}

/// BoundNames of a declaration statement.
pub fn declared_names_of(s: &Stmt) -> Vec<&str> {
    match s {
        Stmt::Var(d) => d
            .declarations
            .iter()
            .flat_map(|d| bound_names(&d.target))
            .collect(),
        Stmt::Function(f) => f.id.iter().map(|i| i.name.as_str()).collect(),
        Stmt::Class(c) => c.id.iter().map(|i| i.name.as_str()).collect(),
        _ => Vec::new(),
    }
}

/// Visits `e` and its subexpressions, descending into arrow functions but
/// not into other functions or class bodies.
pub fn visit_expr(e: &Expr, f: &mut dyn FnMut(&Expr)) {
    f(e);
    let mut v = |x: &Expr| visit_expr(x, f);
    match e {
        Expr::Template(t) => t.exprs.iter().for_each(&mut v),
        Expr::TaggedTemplate { tag, quasi, .. } => {
            v(tag);
            quasi.exprs.iter().for_each(&mut v);
        }
        Expr::Array(elems, _) => {
            for el in elems.iter().flatten() {
                match el {
                    ArrayElem::Expr(x) | ArrayElem::Spread(x) => v(x),
                }
            }
        }
        Expr::Object(props, _) => {
            for p in props {
                match p {
                    ObjectProp::Init(k, x, _) => {
                        visit_key(k, &mut v);
                        v(x)
                    }
                    ObjectProp::CoverInit(_, x, _)
                    | ObjectProp::Spread(x, _)
                    | ObjectProp::Proto(x, _) => v(x),
                    ObjectProp::Method(k, _)
                    | ObjectProp::Getter(k, _)
                    | ObjectProp::Setter(k, _) => visit_key(k, &mut v),
                    ObjectProp::Shorthand(_) => {}
                }
            }
        }
        Expr::Arrow(func) => {
            for p in &func.params {
                if let Some(d) = &p.default {
                    v(d);
                }
            }
            match &func.body {
                FunctionBody::Expr(x) => v(x),
                FunctionBody::Block(stmts) => {
                    stmts.iter().for_each(|s| visit_stmt_exprs(s, &mut v))
                }
            }
        }
        Expr::Class(c) => {
            if let Some(s) = &c.super_class {
                v(s);
            }
        }
        Expr::Unary(_, x, _)
        | Expr::Await(x, _)
        | Expr::Paren(x, _)
        | Expr::OptionalChain(x, _)
        | Expr::PrivateIn(_, x, _) => v(x),
        Expr::Update { target, .. } => v(target),
        Expr::Binary(_, a, b, _) | Expr::Logical(_, a, b, _) => {
            v(a);
            v(b)
        }
        Expr::Conditional {
            test,
            consequent,
            alternate,
            ..
        } => {
            v(test);
            v(consequent);
            v(alternate)
        }
        Expr::Assign { value, .. } => v(value),
        Expr::Sequence(xs, _) => xs.iter().for_each(&mut v),
        Expr::Member {
            object, property, ..
        } => {
            v(object);
            if let MemberProp::Computed(p) = property {
                v(p);
            }
        }
        Expr::SuperMember(MemberProp::Computed(p), _) => v(p),
        Expr::Call { callee, args, .. } | Expr::New { callee, args, .. } => {
            v(callee);
            for a in args {
                match a {
                    Arg::Expr(x) | Arg::Spread(x) => v(x),
                }
            }
        }
        Expr::SuperCall(args, _) => {
            for a in args {
                match a {
                    Arg::Expr(x) | Arg::Spread(x) => v(x),
                }
            }
        }
        Expr::Import {
            specifier, options, ..
        } => {
            v(specifier);
            if let Some(o) = options {
                v(o);
            }
        }
        Expr::Yield {
            argument: Some(x), ..
        } => v(x),
        _ => {}
    }
}

fn visit_key(k: &PropKey, v: &mut dyn FnMut(&Expr)) {
    if let PropKey::Computed(e) = k {
        v(e);
    }
}

/// Visits the expressions directly inside a statement (and nested statements).
pub fn visit_stmt_exprs(s: &Stmt, v: &mut dyn FnMut(&Expr)) {
    match s {
        Stmt::Var(d) => d
            .declarations
            .iter()
            .filter_map(|d| d.init.as_ref())
            .for_each(&mut *v),
        Stmt::Expr(e, _) | Stmt::Throw(e, _) => v(e),
        Stmt::Return(Some(e), _) => v(e),
        Stmt::Block(b) => b.body.iter().for_each(|s| visit_stmt_exprs(s, v)),
        Stmt::If {
            test,
            consequent,
            alternate,
            ..
        } => {
            v(test);
            visit_stmt_exprs(consequent, v);
            if let Some(a) = alternate {
                visit_stmt_exprs(a, v);
            }
        }
        Stmt::For {
            init,
            test,
            update,
            body,
            ..
        } => {
            match init {
                Some(ForInit::Expr(e)) => v(e),
                Some(ForInit::Var(d)) => d
                    .declarations
                    .iter()
                    .filter_map(|d| d.init.as_ref())
                    .for_each(&mut *v),
                None => {}
            }
            if let Some(t) = test {
                v(t);
            }
            if let Some(u) = update {
                v(u);
            }
            visit_stmt_exprs(body, v);
        }
        Stmt::ForIn { right, body, .. } | Stmt::ForOf { right, body, .. } => {
            v(right);
            visit_stmt_exprs(body, v);
        }
        Stmt::While { test, body, .. } | Stmt::DoWhile { test, body, .. } => {
            v(test);
            visit_stmt_exprs(body, v);
        }
        Stmt::With { object, body, .. } => {
            v(object);
            visit_stmt_exprs(body, v);
        }
        Stmt::Labeled { body, .. } => visit_stmt_exprs(body, v),
        Stmt::Switch {
            discriminant,
            cases,
            ..
        } => {
            v(discriminant);
            for c in cases {
                if let Some(t) = &c.test {
                    v(t);
                }
                c.body.iter().for_each(|s| visit_stmt_exprs(s, v));
            }
        }
        Stmt::Try {
            block,
            handler,
            finalizer,
            ..
        } => {
            block.body.iter().for_each(|s| visit_stmt_exprs(s, v));
            if let Some(h) = handler {
                h.body.body.iter().for_each(|s| visit_stmt_exprs(s, v));
            }
            if let Some(f) = finalizer {
                f.body.iter().for_each(|s| visit_stmt_exprs(s, v));
            }
        }
        _ => {}
    }
}
