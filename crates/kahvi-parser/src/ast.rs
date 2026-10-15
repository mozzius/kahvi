//! The abstract syntax tree. Names follow the spec's grammar where practical.
//!
//! Every node that can appear in an error message or in
//! `Function.prototype.toString` carries a [`Span`] of byte offsets into the
//! source text.

use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span {
            start: start as u32,
            end: end as u32,
        }
    }
    pub fn to(self, other: Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

/// A UTF-16 string value, as JavaScript strings are. May contain lone surrogates.
pub type Utf16 = Vec<u16>;

#[derive(Debug, Clone)]
pub struct Script {
    pub body: Vec<Stmt>,
    pub strict: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub body: Vec<ModuleItem>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ModuleItem {
    Import(ImportDecl),
    Export(ExportDecl),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    /// `import x, * as ns from "m"`; empty for `import "m"`.
    pub default: Option<Ident>,
    pub namespace: Option<Ident>,
    pub named: Vec<ImportSpecifier>,
    pub source: Utf16,
    pub attributes: Vec<ImportAttribute>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportSpecifier {
    /// The exported name in the other module (identifier or string).
    pub imported: ModuleExportName,
    pub local: Ident,
}

#[derive(Debug, Clone)]
pub struct ImportAttribute {
    pub key: Utf16,
    pub value: Utf16,
}

/// IdentifierName or a string literal (arbitrary module namespace names).
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleExportName {
    Ident(String),
    String(Utf16),
}

#[derive(Debug, Clone)]
pub enum ExportDecl {
    /// `export { a, b as c }` or `export { a } from "m"`.
    Named {
        specifiers: Vec<ExportSpecifier>,
        source: Option<Utf16>,
        attributes: Vec<ImportAttribute>,
        span: Span,
    },
    /// `export * from "m"` / `export * as ns from "m"`.
    All {
        alias: Option<ModuleExportName>,
        source: Utf16,
        attributes: Vec<ImportAttribute>,
        span: Span,
    },
    /// `export var/let/const/function/class ...`.
    Decl(Stmt),
    /// `export default function/class ...` (possibly anonymous).
    DefaultDecl(Stmt),
    /// `export default <expression>;`
    DefaultExpr(Expr, Span),
}

#[derive(Debug, Clone)]
pub struct ExportSpecifier {
    pub local: ModuleExportName,
    pub exported: ModuleExportName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarKind {
    Var,
    Let,
    Const,
    Using,
    AwaitUsing,
}

impl VarKind {
    pub fn is_lexical(self) -> bool {
        self != VarKind::Var
    }
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub kind: VarKind,
    pub declarations: Vec<VarDeclarator>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VarDeclarator {
    pub target: Pattern,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Var(VarDecl),
    Function(Rc<Function>),
    Class(Rc<Class>),
    Expr(Expr, Span),
    Block(Block),
    Empty(Span),
    If {
        test: Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
        span: Span,
    },
    For {
        init: Option<ForInit>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
        span: Span,
    },
    ForIn {
        left: ForHead,
        right: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    ForOf {
        left: ForHead,
        right: Expr,
        body: Box<Stmt>,
        is_await: bool,
        span: Span,
    },
    While {
        test: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    DoWhile {
        body: Box<Stmt>,
        test: Expr,
        span: Span,
    },
    Continue(Option<Ident>, Span),
    Break(Option<Ident>, Span),
    Return(Option<Expr>, Span),
    With {
        object: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    Switch {
        discriminant: Expr,
        cases: Vec<SwitchCase>,
        span: Span,
    },
    Labeled {
        label: Ident,
        body: Box<Stmt>,
        span: Span,
    },
    Throw(Expr, Span),
    Try {
        block: Block,
        handler: Option<CatchClause>,
        finalizer: Option<Block>,
        span: Span,
    },
    Debugger(Span),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Var(v) => v.span,
            Stmt::Function(f) => f.span,
            Stmt::Class(c) => c.span,
            Stmt::Block(b) => b.span,
            Stmt::Expr(_, s)
            | Stmt::Empty(s)
            | Stmt::Continue(_, s)
            | Stmt::Break(_, s)
            | Stmt::Return(_, s)
            | Stmt::Throw(_, s)
            | Stmt::Debugger(s) => *s,
            Stmt::If { span, .. }
            | Stmt::For { span, .. }
            | Stmt::ForIn { span, .. }
            | Stmt::ForOf { span, .. }
            | Stmt::While { span, .. }
            | Stmt::DoWhile { span, .. }
            | Stmt::With { span, .. }
            | Stmt::Switch { span, .. }
            | Stmt::Labeled { span, .. }
            | Stmt::Try { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ForInit {
    Var(VarDecl),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum ForHead {
    /// `for (var x of ...)`, `for (let [a, b] in ...)`; `init` is the Annex B
    /// `for (var x = 1 in o)` initializer.
    Var {
        kind: VarKind,
        target: Pattern,
        init: Option<Expr>,
    },
    /// `for (x.y of ...)`, `for ([a, b] of ...)`.
    Assign(Pattern),
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    /// `None` for `default:`.
    pub test: Option<Expr>,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CatchClause {
    pub param: Option<Pattern>,
    pub body: Block,
    pub span: Span,
}

/// Binding and assignment targets.
#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(Ident),
    /// Only in assignment patterns: `a.b`, `a[b]`, `a.#b`, `super.x`.
    Expr(Box<Expr>),
    Object {
        props: Vec<ObjectPatternProp>,
        rest: Option<Box<Pattern>>,
        span: Span,
    },
    Array {
        elements: Vec<Option<PatternElem>>,
        rest: Option<Box<Pattern>>,
        span: Span,
    },
}

impl Pattern {
    pub fn span(&self) -> Span {
        match self {
            Pattern::Ident(i) => i.span,
            Pattern::Expr(e) => e.span(),
            Pattern::Object { span, .. } | Pattern::Array { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternElem {
    pub target: Pattern,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct ObjectPatternProp {
    pub key: PropKey,
    pub value: PatternElem,
    /// `{ a }` / `{ a = 1 }` rather than `{ a: b }`.
    pub shorthand: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PropKey {
    Ident(String, Span),
    String(Utf16, Span),
    Number(f64, Span),
    BigInt(String, Span),
    Computed(Box<Expr>),
    Private(String, Span),
}

impl PropKey {
    /// The PropName static semantics: the key's string value if it is not computed.
    pub fn static_name(&self) -> Option<Utf16> {
        match self {
            PropKey::Ident(s, _) => Some(s.encode_utf16().collect()),
            PropKey::String(s, _) => Some(s.clone()),
            PropKey::Number(n, _) => {
                Some(crate::number::number_to_string(*n).encode_utf16().collect())
            }
            PropKey::BigInt(s, _) => Some(s.encode_utf16().collect()),
            PropKey::Computed(_) | PropKey::Private(..) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Delete,
    Void,
    Typeof,
    Plus,
    Minus,
    BitNot,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOp {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Shl,
    Shr,
    UShr,
    BitAnd,
    BitOr,
    BitXor,
    Eq,
    NotEq,
    StrictEq,
    StrictNotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    In,
    InstanceOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
    Coalesce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,
    /// `+=` etc.
    Binary(BinaryOp),
    /// `&&=`, `||=`, `??=`.
    Logical(LogicalOp),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    This(Span),
    Null(Span),
    Bool(bool, Span),
    Number(f64, Span),
    /// Digits without the trailing `n`, already converted to decimal.
    BigInt(String, Span),
    String(Utf16, Span),
    RegExp {
        pattern: Utf16,
        flags: String,
        span: Span,
    },
    Template(Template),
    TaggedTemplate {
        tag: Box<Expr>,
        quasi: Template,
        span: Span,
    },
    Array(Vec<Option<ArrayElem>>, Span),
    Object(Vec<ObjectProp>, Span),
    Function(Rc<Function>),
    Arrow(Rc<Function>),
    Class(Rc<Class>),
    Unary(UnaryOp, Box<Expr>, Span),
    Update {
        op: UpdateOp,
        prefix: bool,
        target: Box<Expr>,
        span: Span,
    },
    Binary(BinaryOp, Box<Expr>, Box<Expr>, Span),
    /// `#x in obj`
    PrivateIn(String, Box<Expr>, Span),
    Logical(LogicalOp, Box<Expr>, Box<Expr>, Span),
    Conditional {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
        span: Span,
    },
    Assign {
        op: AssignOp,
        target: Box<Pattern>,
        value: Box<Expr>,
        span: Span,
    },
    Sequence(Vec<Expr>, Span),
    Member {
        object: Box<Expr>,
        property: MemberProp,
        /// `a?.b`: this link is the start of an optional chain.
        optional: bool,
        span: Span,
    },
    SuperMember(MemberProp, Span),
    Call {
        callee: Box<Expr>,
        args: Vec<Arg>,
        optional: bool,
        span: Span,
    },
    SuperCall(Vec<Arg>, Span),
    New {
        callee: Box<Expr>,
        args: Vec<Arg>,
        span: Span,
    },
    /// The whole of `a?.b.c()`: short-circuits to undefined.
    OptionalChain(Box<Expr>, Span),
    NewTarget(Span),
    ImportMeta(Span),
    /// `import(specifier, options?)`
    Import {
        specifier: Box<Expr>,
        options: Option<Box<Expr>>,
        span: Span,
    },
    Yield {
        argument: Option<Box<Expr>>,
        delegate: bool,
        span: Span,
    },
    Await(Box<Expr>, Span),
    /// Kept so early errors and NamedEvaluation can tell `(a)` from `a`.
    Paren(Box<Expr>, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Ident(i) => i.span,
            Expr::Template(t) => t.span,
            Expr::Function(f) | Expr::Arrow(f) => f.span,
            Expr::Class(c) => c.span,
            Expr::This(s)
            | Expr::Null(s)
            | Expr::Bool(_, s)
            | Expr::Number(_, s)
            | Expr::BigInt(_, s)
            | Expr::String(_, s)
            | Expr::Array(_, s)
            | Expr::Object(_, s)
            | Expr::Unary(_, _, s)
            | Expr::Binary(_, _, _, s)
            | Expr::PrivateIn(_, _, s)
            | Expr::Logical(_, _, _, s)
            | Expr::Sequence(_, s)
            | Expr::SuperMember(_, s)
            | Expr::SuperCall(_, s)
            | Expr::OptionalChain(_, s)
            | Expr::NewTarget(s)
            | Expr::ImportMeta(s)
            | Expr::Await(_, s)
            | Expr::Paren(_, s) => *s,
            Expr::RegExp { span, .. }
            | Expr::TaggedTemplate { span, .. }
            | Expr::Update { span, .. }
            | Expr::Conditional { span, .. }
            | Expr::Assign { span, .. }
            | Expr::Member { span, .. }
            | Expr::Call { span, .. }
            | Expr::New { span, .. }
            | Expr::Import { span, .. }
            | Expr::Yield { span, .. } => *span,
        }
    }

    /// Strips parentheses.
    pub fn unparen(&self) -> &Expr {
        match self {
            Expr::Paren(e, _) => e.unparen(),
            e => e,
        }
    }

    /// IsAnonymousFunctionDefinition.
    pub fn is_anonymous_function_definition(&self) -> bool {
        match self.unparen() {
            Expr::Function(f) => f.id.is_none(),
            Expr::Arrow(_) => true,
            Expr::Class(c) => c.id.is_none(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemberProp {
    Ident(String, Span),
    Computed(Box<Expr>),
    Private(String, Span),
}

#[derive(Debug, Clone)]
pub enum Arg {
    Expr(Expr),
    Spread(Expr),
}

#[derive(Debug, Clone)]
pub enum ArrayElem {
    Expr(Expr),
    Spread(Expr),
}

#[derive(Debug, Clone)]
pub struct Template {
    /// Cooked strings; `None` where a tagged template had an invalid escape.
    pub cooked: Vec<Option<Utf16>>,
    pub raw: Vec<Utf16>,
    pub exprs: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ObjectProp {
    /// `key: value`
    Init(PropKey, Expr, Span),
    /// `{ a }`
    Shorthand(Ident),
    /// `{ a = 1 }`: only valid once reinterpreted as a pattern.
    CoverInit(Ident, Expr, Span),
    Method(PropKey, Rc<Function>),
    Getter(PropKey, Rc<Function>),
    Setter(PropKey, Rc<Function>),
    Spread(Expr, Span),
    /// `__proto__: value` (not computed, not shorthand)
    Proto(Expr, Span),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    Normal,
    Arrow,
    Method,
    Getter,
    Setter,
    ClassConstructor,
    DerivedConstructor,
    /// The synthetic function evaluating a class field initializer or static block.
    ClassFieldInit,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub target: Pattern,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum FunctionBody {
    Block(Vec<Stmt>),
    /// Concise arrow body.
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Option<Ident>,
    pub kind: FunctionKind,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: Vec<Param>,
    pub rest: Option<Pattern>,
    pub body: FunctionBody,
    /// Strict mode code (inherited or from a "use strict" directive).
    pub strict: bool,
    /// The body starts with a "use strict" directive.
    pub has_use_strict: bool,
    pub simple_params: bool,
    /// The whole source text (for `Function.prototype.toString`).
    pub span: Span,
    /// Facts the compiler needs and the parser already knows.
    pub uses_arguments: bool,
    pub uses_this: bool,
    pub has_direct_eval: bool,
    /// For the expected-argument-count `length` property.
    pub expected_args: u32,
    /// Parsed with lazy compilation: the body was checked for early errors
    /// and dropped (`params`, `rest` and `body` are empty). It is parsed
    /// again from the source when the function is first called.
    pub lazy: Option<LazyFunction>,
}

/// What reparsing a lazy function needs besides its source position.
#[derive(Debug, Clone, Copy)]
pub struct LazyFunction {
    /// The code around the function is strict mode code.
    pub outer_strict: bool,
    /// Parsed with the Module goal.
    pub module: bool,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub id: Option<Ident>,
    pub super_class: Option<Box<Expr>>,
    pub constructor: Option<Rc<Function>>,
    pub elements: Vec<ClassElement>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    Method,
    Getter,
    Setter,
}

#[derive(Debug, Clone)]
pub enum ClassElement {
    Method {
        key: PropKey,
        kind: MethodKind,
        is_static: bool,
        function: Rc<Function>,
    },
    Field {
        key: PropKey,
        is_static: bool,
        /// Wrapped as a ClassFieldInit function so it gets its own `this`.
        init: Option<Rc<Function>>,
        span: Span,
    },
    /// `accessor x = 1`
    Accessor {
        key: PropKey,
        is_static: bool,
        init: Option<Rc<Function>>,
        span: Span,
    },
    StaticBlock(Rc<Function>),
}
