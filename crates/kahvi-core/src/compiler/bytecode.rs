//! The bytecode: a stack machine. Each function body compiles to one
//! [`FunctionCode`]; nested functions are in [`FunctionCode::functions`].
//!
//! Stack effects are written `[before] -> [after]`, top of stack last.

use std::sync::Arc;

use crate::function::ThisMode;
use crate::object::PropertyKey;
use crate::value::{BigInt, JsString};

/// Index of an instruction in [`FunctionCode::ops`].
pub type Label = u32;

/// (`repr(u32)`: an explicit tag the interpreter dispatches on directly.)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum Op {
    // ---- constants -------------------------------------------------------
    /// [] -> [undefined]
    Undefined,
    Null,
    True,
    False,
    Number(f64),
    String(JsString),
    BigInt(BigInt),

    // ---- stack -----------------------------------------------------------
    /// [a] -> []
    Pop,
    /// [a] -> [a a]
    Dup,
    /// [a b] -> [a b a b]
    Dup2,
    /// [a b] -> [b a]
    Swap,
    /// [a b c] -> [c a b]
    RotRight3,
    /// [a b c d] -> [d a b c]
    RotRight4,
    /// [a b c d e] -> [e a b c d]
    RotRight5,
    /// [.. x (n values)] -> [.. x (n values) x]: copies the value `n` below the top.
    Pick(u32),

    // ---- completion value (script and eval code) ---------------------------
    /// [v] -> []: the completion value of the code so far is `v`.
    SetCompletion,
    /// [] -> [v]: the completion value of the code so far.
    GetCompletion,

    // ---- identifiers -------------------------------------------------------
    /// [] -> [value]: ResolveBinding(name) then GetValue. The `u32` of the
    /// name instructions is their binding cache ([`FunctionCode::name_caches`],
    /// [`NO_NAME_CACHE`] for none).
    GetName(JsString, u32),
    /// [] -> [typeof value]: like GetName but unresolvable gives "undefined".
    TypeofName(JsString, u32),
    /// [] -> [func this]: GetName plus the call's this value (with-object or undefined).
    GetNameForCall(JsString, u32),
    /// [] -> []: ResolveBinding(name), pushed on the reference stack.
    ResolveName(JsString, u32),
    /// [] -> [value]: GetValue of the top reference (it stays).
    GetRef,
    /// [value] -> [value]: PutValue to the top reference, which is popped.
    PutRef,
    /// [] -> []: pops the top reference without using it.
    DropRef,
    /// [value] -> []: InitializeReferencedBinding in the running lexical environment.
    InitName(JsString),
    /// [] -> [bool]: `delete name`.
    DeleteName(JsString),
    /// Annex B.3.2: the evaluation of a block-level FunctionDeclaration whose
    /// name got a var binding at declaration instantiation: the var binding
    /// is set to the block's function object.
    AnnexBFunction(JsString),

    // ---- environments ------------------------------------------------------
    /// BlockDeclarationInstantiation of [`FunctionCode::scopes`]`[i]` in a new
    /// declarative environment, which becomes the running lexical environment.
    PushScope(u32),
    /// A new empty declarative environment (e.g. for a class body).
    PushEmptyScope,
    /// Restores the outer environment of the running lexical environment.
    PopScope,
    /// CreatePerIterationEnvironment: copies the running lexical environment.
    CopyScope,
    /// [obj] -> []: a `with` object environment becomes the running one.
    PushWith,

    // ---- frame locals ------------------------------------------------------
    // The bindings of code with `FunctionCode::locals` live in its frame,
    // one slot each, instead of in environment records; these instructions
    // replace the name instructions for them.
    /// [] -> [value]: GetValue of local `slot` (a ReferenceError before its
    /// initialization).
    GetLocal(u32),
    /// [] -> []: a reference to local `slot`, pushed on the reference stack.
    ResolveLocal(u32),
    /// [value] -> []: InitializeReferencedBinding of local `slot`.
    InitLocal(u32),
    /// Locals `start..start + count` become uninitialized: the bindings of a
    /// scope entered again (a loop body) start out in their dead zone.
    ClearLocals(u32, u32),

    // ---- properties --------------------------------------------------------
    /// [base] -> [value]. The `u32` of the named property instructions is
    /// their cache ([`FunctionCode::prop_caches`]), numbered when the code
    /// is finished (emitted as 0).
    GetProp(PropertyKey, u32),
    /// [base key] -> [value]: GetValue; ToObject(base) happens before ToPropertyKey(key).
    GetElem,
    /// [base key] -> [base propertyKey value]: GetElem keeping the converted key
    /// for a following SetElem (compound assignment, update expressions).
    GetElemForUpdate,
    /// [base] -> [func base]: method lookup for calls.
    GetPropForCall(PropertyKey, u32),
    /// [base key] -> [func base]
    GetElemForCall,
    /// [base value] -> [value]: PutValue to base.key.
    SetProp(PropertyKey, u32),
    /// [base key value] -> [value]
    SetElem,
    /// [base] -> [bool]
    DeleteProp(PropertyKey),
    /// [base key] -> [bool]
    DeleteElem,
    /// [key] -> [property key value]: ToPropertyKey (as a String or Symbol).
    ToPropertyKey,
    /// [base] -> [base]: throws a TypeError if base is undefined or null.
    RequireObjectCoercible,

    // ---- super and private references -------------------------------------
    /// [] -> [value]: super.name (the whole SuperProperty evaluation and GetValue).
    GetSuperProp(PropertyKey),
    /// [] -> [func this]: super.name as a callee.
    GetSuperPropForCall(PropertyKey),
    /// [] -> [base]: GetSuperBase(env) of MakeSuperPropertyReference. A Super
    /// Reference on the stack is [this key base]: `This`, then the key value,
    /// then `SuperBase`.
    SuperBase,
    /// [this key base] -> [value]: GetValue of a Super Reference.
    GetSuperElem,
    /// [this key base] -> [func this]
    GetSuperElemForCall,
    /// [this key base] -> [this key' base' value]: GetValue, keeping the reference
    /// (key converted to a property key, base to an object) for a PutValue.
    GetSuperElemForUpdate,
    /// [this key base value] -> [value]: PutValue of a Super Reference.
    SetSuperElem,
    /// [base] -> [value]: PrivateGet(base, #name).
    GetPrivate(JsString),
    /// [base value] -> [value]
    SetPrivate(JsString),
    /// [base] -> [func base]
    GetPrivateForCall(JsString),
    /// [obj] -> [bool]: `#name in obj`.
    PrivateIn(JsString),

    // ---- operators ---------------------------------------------------------
    /// [a b] -> [a op b]
    Binary(BinOp),
    /// [a] -> [-a]
    Neg,
    /// [a] -> [+a] (ToNumber)
    Plus,
    /// [a] -> [~a]
    BitNot,
    /// [a] -> [!a]
    Not,
    /// [a] -> [typeof a]
    Typeof,
    /// [a] -> [ToNumeric(a)]
    ToNumeric,
    /// [n] -> [n + 1] for a Number or BigInt
    Inc,
    /// [n] -> [n - 1]
    Dec,
    /// [a] -> [ToString(a)] (template literals)
    ToString,

    // ---- control flow ------------------------------------------------------
    Jump(Label),
    /// [v] -> []
    JumpIfFalse(Label),
    /// [v] -> []
    JumpIfTrue(Label),
    /// [v] -> [v]: jumps if ToBoolean(v) is false; never pops.
    JumpIfFalseKeep(Label),
    JumpIfTrueKeep(Label),
    /// [v] -> [v]: jumps if v is undefined or null.
    JumpIfNullishKeep(Label),
    /// [v] -> [v]: jumps unless v is undefined or null.
    JumpIfNotNullishKeep(Label),
    /// [v] -> [v]: jumps unless v is undefined.
    JumpIfNotUndefined(Label),

    // ---- exceptions and finally ----------------------------------------------
    /// [v] -> throws v
    Throw,
    /// Throws a new TypeError/ReferenceError/SyntaxError with a message from
    /// [`FunctionCode::messages`].
    ThrowError(ErrorType, u32),
    /// Installs an exception handler: on a throw, the stacks unwind to their
    /// current heights, the lexical environment is restored, the thrown
    /// value is pushed and execution continues at the label.
    PushHandler(Label),
    PopHandler,
    /// [value kind] -> []: ends a finally block. `kind` is a Number:
    /// 0 normal (value dropped, fall through), 1 throw (rethrow value),
    /// n >= 2 jump to `FunctionCode::finally_targets[i][n - 2]` with value pushed.
    EndFinally(u32),

    // ---- explicit resource management ------------------------------------------
    /// [v] -> []: AddDisposableResource(env.[[DisposableResourceStack]], v, kind)
    /// on the running LexicalEnvironment; `true` is ~async-dispose~.
    AddDisposable(bool),
    /// [value kind] -> [state]: starts DisposeResources(env.[[DisposableResourceStack]],
    /// completion) for the running LexicalEnvironment, where [value kind] is a
    /// finally block's completion.
    DisposeStart,
    /// [state] -> [state] (jumps when done) | [state v]: runs DisposeResources
    /// to its next Await, pushing the value to await.
    DisposeStep(Label),
    /// [state e] -> [state]: the awaited value of an async dispose method threw e.
    DisposeAwaitThrew,
    /// [state] -> [value kind]: DisposeResources' result as a finally completion.
    DisposeEnd,

    // ---- functions and calls -------------------------------------------------
    /// [] -> [closure of functions[i]] (InstantiateOrdinaryFunctionExpression etc.),
    /// named after its own BindingIdentifier or "".
    Closure(u32),
    /// [name] -> [closure]: a closure named by NamedEvaluation (`name` is a
    /// property key value: a String, or a Symbol giving "[description]").
    ClosureNamed(u32),
    /// [] -> [name]: the [[ClassFieldInitializerName]] of the running function.
    FieldInitializerName,
    /// [func this args...] -> [result]
    Call(u32),
    /// [func this argsArray] -> [result]
    CallSpread,
    /// [func this args...] -> [result]: a call that is a direct eval if func is %eval%.
    CallEval(u32),
    /// [func this argsArray] -> [result]
    CallEvalSpread,
    /// [ctor args...] -> [object]
    New(u32),
    /// [ctor argsArray] -> [object]
    NewSpread,
    /// [] -> [func]: GetSuperConstructor() (before the arguments are evaluated).
    GetSuperConstructor,
    /// [func args...] -> [this]
    SuperCall(u32),
    /// [func argsArray] -> [this]
    SuperCallSpread,
    /// [v] -> returns v (after the frame's own completion handling)
    Return,
    /// [] -> [this]: ResolveThisBinding.
    This,
    /// [] -> [new.target]
    NewTarget,
    /// [] -> [import.meta]
    ImportMeta,
    /// [specifier options] -> [promise]
    ImportCall,
    /// [] -> [args[i] or undefined]: the i-th argument of the running call.
    GetArg(u32),
    /// [] -> [array of args[i..]]
    GetRestArgs(u32),
    /// FunctionDeclarationInstantiation steps after parameter binding.
    InstantiateFunctionBody,

    // ---- object and array literals -------------------------------------------
    /// [] -> [new ordinary object]
    NewObject,
    /// [] -> [new empty array]
    NewArray,
    /// [obj value] -> [obj]: CreateDataPropertyOrThrow(obj, key, value).
    DefineField(PropertyKey),
    /// [obj key value] -> [obj]
    DefineFieldComputed,
    /// [array value] -> [array]: appends value.
    ArrayPush,
    /// [array] -> [array]: appends a hole (an elision).
    ArrayHole,
    /// [array iterable] -> [array]: appends every value of the iterable.
    ArraySpread,
    /// [obj source] -> [obj]: CopyDataProperties(obj, source, «»).
    CopyDataProperties,
    /// [k1 .. kn target source] -> [target]: CopyDataProperties(target,
    /// source, «k1 .. kn») (object rest patterns).
    CopyDataPropertiesExcluding(u32),
    /// [obj proto] -> [obj]: `__proto__: value` in an object literal.
    SetPrototypeLiteral,
    /// [obj key closure] -> [obj]: defines a method/getter/setter/field on obj
    /// (the closure's [[HomeObject]] is set to obj).
    DefineMethod(MethodDef),
    /// [] -> [template object of templates[i]]
    GetTemplateObject(u32),
    /// [] -> [new RegExp]
    RegExp(JsString, JsString),

    // ---- iteration -------------------------------------------------------------
    /// [iterable] -> []: GetIterator(sync), pushed on the iterator stack.
    GetIterator,
    /// [iterable] -> []: GetIterator(async).
    GetAsyncIterator,
    /// [] -> [value] or jumps when done: IteratorStepValue of the top iterator.
    IteratorNext(Label),
    /// [] -> [value]: IteratorStepValue for destructuring (undefined once done).
    IteratorNextOrUndefined,
    /// [] -> []: IteratorStep for an elision (the result's value is not read).
    IteratorSkip,
    /// [] -> [array]: the remaining values of the top iterator.
    IteratorRest,
    /// [] -> []: pops the top iterator without closing it.
    PopIterator,
    /// [] -> []: IteratorClose(top, normal completion), then pops it.
    IteratorClose,
    /// [exception] -> [exception]: IteratorClose(top, throw completion) if not
    /// done, pops it; the exception is rethrown by the following Throw.
    IteratorCloseForThrow,
    /// [] -> [result]: Call(next, iterator) of the top (async) iterator, unchecked.
    IteratorCallNext,
    /// [result] -> [value], or jumps when done: the result-object checks of
    /// an awaited `for await` step (not an Object is a TypeError).
    IteratorResultUnwrap(Label),
    /// [] -> [innerResult], or jumps: AsyncIteratorClose for a normal completion.
    /// Pops the top iterator; jumps when it has no `return` method. Followed by
    /// Await and IteratorCheckObject.
    AsyncIteratorClose(Label),
    /// [exception] -> [exception innerResult], or jumps: AsyncIteratorClose for a
    /// throw completion. Pops the top iterator; jumps (ignoring errors) when there
    /// is nothing to await.
    AsyncIteratorCloseForThrow(Label),
    /// [v] -> []: TypeError unless v is an Object (iterator.return() results).
    IteratorCheckObject,
    /// [obj] -> []: EnumerateObjectProperties(ToObject(obj)) on the iterator stack
    /// (for undefined/null pushes an empty iteration).
    ForInStart,
    /// [] -> [key] or jumps when done.
    ForInNext(Label),

    // ---- generators and async --------------------------------------------------
    /// [] -> []: initial suspension of a generator (after FunctionDeclarationInstantiation).
    GeneratorStart,
    /// [value] -> [received value] or a resumption with return/throw.
    /// On resume, a return resumption jumps to the label with the value pushed.
    Yield(Label),
    /// [iterable] -> [result]: `yield*`; a return resumption jumps to the label.
    YieldDelegate(Label),
    /// [value] -> [awaited value]
    Await,
    /// `yield*` in async generators (§15.5.5 with generatorKind async): the
    /// resumption kind (0 normal, 1 throw, 2 return) lives in the frame.
    SetDelegateKind(u8),
    JumpIfDelegateKind(u8, Label),
    /// [received] -> [innerResult]: Call(next) or Call(throw) on the delegate;
    /// jumps (stack unchanged) when a throw resumption finds no `throw` method.
    AsyncDelegateCall(Label),
    /// [received] -> [innerReturnResult]: Call(return); jumps (stack unchanged)
    /// when there is no `return` method.
    AsyncDelegateReturn(Label),
    /// [innerResult] -> [value]: the result checks; when done, pops the
    /// delegate iterator and jumps.
    AsyncDelegateResult(Label),
    /// [value] -> [received]: AsyncGeneratorYield(value); the resumption
    /// kind is stored for the loop.
    DelegateYield,

    // ---- classes -----------------------------------------------------------------
    /// [superclass] -> [proto ctorParent] (or [] -> [proto ctorParent] when
    /// the class has no `extends`): the heritage steps of ClassDefinitionEvaluation.
    ClassHeritage(bool),
    /// [name proto ctorParent] -> [ctor proto]: creates the constructor F from
    /// functions[i] (or a default constructor when `None`) and names it.
    CreateClass(ClassInfo),
    /// Creates the class's PrivateEnvironment with the names of classes[i].
    PushPrivateScope(u32),
    PopPrivateScope,
    /// [ctor proto key init] -> [ctor proto]: an instance or static field
    /// (`init` is a closure or undefined); static blocks have no key: [ctor proto closure].
    DefineClassField(FieldDef),
    /// [ctor proto] -> [ctor proto]: a private method/accessor; closure on top.
    DefinePrivateMethod(PrivateMethodDef),
    /// [ctor] -> [ctor]: runs the static fields and blocks of the class.
    FinishClass,
    /// [] -> [this]: the body of a default derived constructor: super(...args)
    /// forwarding the arguments without iterating them.
    SuperCallForward,

    // ---- misc -----------------------------------------------------------------
    /// Throws an uncatchable "not implemented" error naming messages[i].
    NotImplemented(u32),
    Debugger,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
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
pub enum ErrorType {
    Type,
    Reference,
    Syntax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    Method,
    Getter,
    Setter,
}

/// How [`Op::DefineMethod`] defines its property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodDef {
    pub kind: MethodKind,
    /// Enumerable (object literals) or not (classes).
    pub enumerable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassInfo {
    /// Index into `functions` of the explicit constructor, if any.
    pub constructor: Option<u32>,
    /// Index into `classes`.
    pub class: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldDef {
    pub is_static: bool,
    /// The key is a private name (a String on the stack).
    pub private: bool,
    pub static_block: bool,
    /// An `accessor` field: defines the getter/setter pair as well.
    pub auto_accessor: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivateMethodDef {
    pub name: JsString,
    pub kind: MethodKind,
    pub is_static: bool,
}

/// What kind of code a [`FunctionCode`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind {
    Script,
    Module,
    Eval,
    Function,
}

/// The kind of function, from the parser's FunctionKind plus async/generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionFlavor {
    Normal,
    Arrow,
    Method,
    Getter,
    Setter,
    ClassConstructor,
    DerivedConstructor,
    ClassFieldInit,
}

// Instructions stay two words.
const _: () = assert!(std::mem::size_of::<Op>() == 16);
// Every function has one, called or not (a lazy stub, or decoded with the
// cached code of the function around it).
const _: () = assert!(std::mem::size_of::<FunctionCode>() <= 392);

/// A declaration to instantiate in a scope.
#[derive(Debug, Clone)]
pub struct ScopeInfo {
    /// Lexically declared names: (name, is_const).
    pub lexical: Vec<(JsString, bool)>,
    /// Function declarations to initialize: (name, index into `functions`).
    pub functions: Vec<(JsString, u32)>,
    /// The scope of a Catch clause's parameter.
    pub is_catch: bool,
}

/// What kind of binding a frame local is (`FunctionCode::locals`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalKind {
    /// A formal parameter: initialized by the parameter binding code.
    Param,
    /// A var: initialized to undefined on entry.
    Var,
    /// let, or a catch parameter: uninitialized until its declaration runs.
    Let,
    /// const: like let, and assignment is a TypeError.
    Const,
}

/// A binding kept in a frame slot instead of an environment record.
#[derive(Debug, Clone)]
pub struct LocalInfo {
    pub name: JsString,
    pub kind: LocalKind,
}

/// Declarations of a script, function body or eval code, for the
/// declaration-instantiation algorithms.
#[derive(Debug, Clone, Default)]
pub struct Declarations {
    /// VarDeclaredNames (without function declarations).
    pub var_names: Vec<JsString>,
    /// Var-scoped function declarations to initialize, last one per name wins:
    /// (name, index into `functions`).
    pub functions: Vec<(JsString, u32)>,
    /// Top-level lexical declarations: (name, is_const).
    pub lexical: Vec<(JsString, bool)>,
    /// Annex B.3.2: the names of the block-level function declarations that
    /// may also get a var binding (non-strict code only), once each. The
    /// declaration-instantiation algorithms decide which do.
    pub annex_b_functions: Vec<JsString>,
}

/// Formal parameters, for FunctionDeclarationInstantiation.
#[derive(Debug, Clone, Default)]
pub struct Params {
    /// BoundNames of the formals, in order (with duplicates).
    pub names: Vec<JsString>,
    pub simple: bool,
    pub has_expressions: bool,
    pub has_duplicates: bool,
}

/// A template literal site.
#[derive(Debug, Clone)]
pub struct TemplateInfo {
    /// `None` where a tagged template had an invalid escape.
    pub cooked: Vec<Option<JsString>>,
    pub raw: Vec<JsString>,
    /// Identifies the site for the realm's [[TemplateMap]].
    pub site: u32,
}

/// The static parts of a class definition.
#[derive(Debug, Clone, Default)]
pub struct ClassStatic {
    pub name: Option<JsString>,
    /// Private names declared in the class body.
    pub private_names: Vec<JsString>,
    /// Source span of the class, for the constructor's toString.
    pub span: (u32, u32),
}

/// The strings and BigInts that the code of one compilation unit (a script,
/// module, eval or dynamic function, with every function nested in it)
/// refers to. Each `FunctionCode` of the unit shares one; the collector
/// traces it from whatever holds the code (docs/GC.md).
#[derive(Debug, Default)]
pub struct CodeConstants {
    values: std::sync::OnceLock<Box<[crate::value::Value]>>,
    /// The code compiled later for this unit's lazy functions (each its own
    /// unit), traced with it: a lazy function's code is shared by every
    /// closure made from it.
    lazy: std::sync::Mutex<Vec<Arc<FunctionCode>>>,
}

impl CodeConstants {
    pub(crate) fn set(&self, values: Box<[crate::value::Value]>) {
        self.values
            .set(values)
            .expect("a compilation unit's constants are set once");
    }

    pub fn values(&self) -> &[crate::value::Value] {
        self.values.get().map_or(&[], |v| v)
    }

    pub(crate) fn add_lazy(&self, code: Arc<FunctionCode>) {
        self.lazy.lock().expect("lazy code list").push(code);
    }

    pub(crate) fn lazy_code(&self) -> Vec<Arc<FunctionCode>> {
        self.lazy.lock().expect("lazy code list").clone()
    }
}

impl FunctionCode {
    /// Adds this code and the code nested in it (counted once each, by
    /// address, in `seen`) to `bytes` and `count`: the code's own record
    /// and arrays, for [`crate::gc::HeapStats`].
    pub(crate) fn census(
        self: &Arc<Self>,
        seen: &mut crate::hash::FxHashSet<usize>,
        bytes: &mut usize,
        count: &mut usize,
    ) {
        use std::mem::size_of_val;
        if !seen.insert(Arc::as_ptr(self) as usize) {
            return;
        }
        *count += 1;
        *bytes += std::mem::size_of::<Self>()
            + size_of_val(&*self.ops)
            + size_of_val(&*self.positions)
            + size_of_val(&*self.functions)
            + self.decls.as_ref().map_or(0, |d| size_of_val(&**d))
            + self.tables.as_ref().map_or(0, |t| size_of_val(&**t))
            + size_of_val(self.scopes())
            + size_of_val(self.templates())
            + size_of_val(self.classes())
            + size_of_val(self.strict_ranges())
            + self.locals.as_deref().map_or(0, size_of_val)
            + self.messages().iter().map(|m| m.len() + 24).sum::<usize>()
            + self
                .finally_targets()
                .iter()
                .map(|t| size_of_val(&**t) + 24)
                .sum::<usize>();
        for f in self.functions.iter() {
            f.census(seen, bytes, count);
        }
        if let Some(code) = self.lazy.as_ref().and_then(|l| l.compiled.get()) {
            code.census(seen, bytes, count);
        }
    }
}

/// A function compiled when it is first called (`ast::Function::lazy`):
/// its `FunctionCode` holds only what creating a closure needs.
#[derive(Debug)]
pub struct LazyCode {
    pub function: kahvi_parser::ast::LazyFunction,
    /// The enclosing `with` bodies and direct-eval functions
    /// (`CompileUnit::dynamic_scopes`).
    pub dynamic_scopes: u32,
    pub is_expression: bool,
    pub compiled: std::sync::OnceLock<Arc<FunctionCode>>,
}

/// Compiled code for a script, module, eval or function body.
#[derive(Debug)]
pub struct FunctionCode {
    pub kind: CodeKind,
    pub flavor: FunctionFlavor,
    pub is_async: bool,
    pub is_generator: bool,
    pub strict: bool,
    pub this_mode: ThisMode,
    /// The function's own name (BindingIdentifier), if any.
    pub name: Option<JsString>,
    /// ExpectedArgumentCount, for the `length` property.
    pub length: u32,
    pub params: Params,
    /// The body's declarations (`None`: none; see [`FunctionCode::decls`]).
    pub decls: Option<Box<Declarations>>,
    /// Whether the body needs an arguments object if FunctionDeclarationInstantiation allows it.
    pub uses_arguments: bool,
    /// Whether the code contains a direct eval.
    pub has_direct_eval: bool,
    /// A named function expression binds its own name in a scope between it and its closure.
    pub binds_own_name: bool,
    /// Nothing can keep this function's environments alive past its call:
    /// it creates no closures or classes, has no arguments object, direct
    /// eval or `with`, and is not a generator or async. A call then frees
    /// its environments as they go out of scope instead of leaving them to
    /// the garbage collector (see `Agent::free_env`).
    pub envs_die_with_call: bool,
    /// The bindings of a function whose environments die with its call and
    /// whose bindings no code can reach by name at run time (see
    /// `compiler::function::compile_function`): each is a slot of the
    /// call's frame, and its function environment gets no bindings.
    pub locals: Option<Box<[LocalInfo]>>,
    /// Code with locals that nothing in needs its function environment
    /// (an arrow function, or one without `this`, `super` or `new.target`):
    /// a call runs it in its closure's environment instead, as nothing can
    /// observe the empty environment's absence.
    pub skips_function_env: bool,
    pub ops: Box<[Op]>,
    /// (op index, source offset) pairs, ascending, for error positions.
    pub positions: Box<[(u32, u32)]>,
    pub functions: Box<[Arc<FunctionCode>]>,
    /// The tables few functions need (`None`: all empty; see the accessors
    /// such as [`FunctionCode::scopes`]).
    pub tables: Option<Box<CodeTables>>,
    /// The whole source text and this code's span in it (Function.prototype.toString).
    pub source: Arc<str>,
    pub span: (u32, u32),
    /// Index of the script/module this code belongs to.
    pub script: u32,
    /// The handles the compilation unit's code refers to.
    pub constants: Arc<CodeConstants>,
    /// Where each name instruction found its binding last time: the number
    /// of environments out from the running one and the binding's slot in
    /// it, as `(hops << 32 | slot) + 1`, 0 when unknown. Only instructions
    /// whose environment chain cannot gain bindings at run time (no `with`,
    /// no direct eval in or around them, not eval code) get one: there a
    /// binding, once found, is always found in the same place.
    pub name_caches: Box<[std::sync::atomic::AtomicU64]>,
    /// The caches of the named property instructions (`GetProp`,
    /// `GetPropForCall`, `SetProp`), see [`PropCache`].
    pub prop_caches: Box<[PropCache]>,
    /// Set for a function not compiled yet.
    pub lazy: Option<Box<LazyCode>>,
}

/// The tables of a [`FunctionCode`] that most functions leave empty, kept
/// apart so that those functions do not carry them.
#[derive(Debug, Default)]
pub struct CodeTables {
    pub scopes: Box<[ScopeInfo]>,
    pub templates: Box<[TemplateInfo]>,
    pub classes: Box<[ClassStatic]>,
    pub messages: Box<[String]>,
    pub finally_targets: Box<[Vec<Label>]>,
    /// Instruction ranges `[start, end)` that are strict mode code inside
    /// non-strict code: a class's heritage and element keys (all parts of a
    /// ClassDeclaration or ClassExpression are strict mode code).
    pub strict_ranges: Box<[(u32, u32)]>,
}

impl CodeTables {
    fn is_empty(&self) -> bool {
        self.scopes.is_empty()
            && self.templates.is_empty()
            && self.classes.is_empty()
            && self.messages.is_empty()
            && self.finally_targets.is_empty()
            && self.strict_ranges.is_empty()
    }

    /// The tables, or `None` when all are empty.
    pub(crate) fn boxed(self) -> Option<Box<CodeTables>> {
        (!self.is_empty()).then(|| Box::new(self))
    }
}

static NO_DECLARATIONS: Declarations = Declarations {
    var_names: Vec::new(),
    functions: Vec::new(),
    lexical: Vec::new(),
    annex_b_functions: Vec::new(),
};

/// Where a property instruction last found its property: the position of
/// the key among the named properties of the object that had it, plus 1
/// (0 when unknown). Objects built the same way keep their keys in the same
/// order, so the key is usually at that position again; a lookup checks
/// there first and falls back to a search (see `PropertyMap::get_hinted`).
pub type PropCache = std::sync::atomic::AtomicU32;

/// The cache index of a name instruction that has no binding cache.
pub const NO_NAME_CACHE: u32 = u32::MAX;

impl FunctionCode {
    /// The body's declarations.
    pub fn decls(&self) -> &Declarations {
        self.decls.as_deref().unwrap_or(&NO_DECLARATIONS)
    }

    /// The body's declarations, to fill in while compiling.
    pub(crate) fn decls_mut(&mut self) -> &mut Declarations {
        self.decls.get_or_insert_with(Default::default)
    }

    pub fn scopes(&self) -> &[ScopeInfo] {
        self.tables.as_ref().map_or(&[], |t| &t.scopes)
    }

    pub fn templates(&self) -> &[TemplateInfo] {
        self.tables.as_ref().map_or(&[], |t| &t.templates)
    }

    pub fn classes(&self) -> &[ClassStatic] {
        self.tables.as_ref().map_or(&[], |t| &t.classes)
    }

    pub fn messages(&self) -> &[String] {
        self.tables.as_ref().map_or(&[], |t| &t.messages)
    }

    pub fn finally_targets(&self) -> &[Vec<Label>] {
        self.tables.as_ref().map_or(&[], |t| &t.finally_targets)
    }

    pub fn strict_ranges(&self) -> &[(u32, u32)] {
        self.tables.as_ref().map_or(&[], |t| &t.strict_ranges)
    }

    /// The source text of the function (for Function.prototype.toString).
    pub fn source_text(&self) -> &str {
        self.source
            .get(self.span.0 as usize..self.span.1 as usize)
            .unwrap_or("")
    }

    /// The source offset of the instruction at `pc`, if known.
    pub fn position_of(&self, pc: usize) -> Option<u32> {
        let i = self.positions.partition_point(|&(op, _)| op as usize <= pc);
        i.checked_sub(1).map(|i| self.positions[i].1)
    }
}
