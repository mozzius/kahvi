//! Encoding compiled code as bytes and back, for caching the code of lazily
//! compiled functions between runtimes and runs (`crate::code_cache`).
//!
//! A record holds one [`FunctionCode`] tree (a function with the functions
//! and classes nested in it) and the strings it refers to. What the code
//! takes from its surroundings (the source text, script and constants) comes
//! from the lazy stub it replaces. Code the encoder cannot represent (a
//! BigInt literal, a Symbol key) is not cached.

use std::collections::HashMap;
use std::sync::Arc;

use super::bytecode::*;
use crate::agent::Agent;
use crate::function::ThisMode;
use crate::object::PropertyKey;
use crate::value::{BigInt, JsString};

/// Changes whenever the parser or compiler does (build.rs).
pub const FORMAT: &str = env!("KAHVI_CODE_FORMAT");

pub(crate) struct Writer {
    buf: Vec<u8>,
    strings: Vec<JsString>,
    index: HashMap<JsString, u32>,
}

pub(crate) struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    strings: Vec<JsString>,
}

impl Writer {
    fn u8(&mut self, v: u8) {
        self.buf.push(v);
    }
    fn u32(&mut self, v: u32) {
        // LEB128: most operands are small.
        let mut v = v;
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.buf.push(b);
                return;
            }
            self.buf.push(b | 0x80);
        }
    }
    fn len(&mut self, n: usize) -> Option<()> {
        self.u32(u32::try_from(n).ok()?);
        Some(())
    }
}

impl Reader<'_> {
    fn u8(&mut self) -> Option<u8> {
        let b = *self.buf.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
    fn u32(&mut self) -> Option<u32> {
        let mut v: u32 = 0;
        for shift in (0..35).step_by(7) {
            let b = self.u8()?;
            v |= u32::from(b & 0x7f).checked_shl(shift)?;
            if b & 0x80 == 0 {
                return Some(v);
            }
        }
        None
    }
    fn len(&mut self) -> Option<usize> {
        let n = self.u32()? as usize;
        // Every element takes at least a byte: rejects corrupt lengths
        // before allocating for them.
        (n <= self.buf.len() - self.pos).then_some(n)
    }
}

/// A value the codec can write and read.
trait Code: Sized {
    fn put(&self, w: &mut Writer) -> Option<()>;
    fn get(r: &mut Reader) -> Option<Self>;
}

impl Code for u8 {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.u8(*self);
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        r.u8()
    }
}

impl Code for u32 {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.u32(*self);
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        r.u32()
    }
}

impl Code for bool {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.u8(u8::from(*self));
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        match r.u8()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl Code for f64 {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.buf.extend_from_slice(&self.to_bits().to_le_bytes());
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        let bytes = r.buf.get(r.pos..r.pos + 8)?;
        r.pos += 8;
        Some(f64::from_bits(u64::from_le_bytes(bytes.try_into().ok()?)))
    }
}

impl Code for JsString {
    fn put(&self, w: &mut Writer) -> Option<()> {
        let next = w.strings.len() as u32;
        let i = *w.index.entry(*self).or_insert(next);
        if i == next {
            w.strings.push(*self);
        }
        w.u32(i);
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        let i = r.u32()? as usize;
        r.strings.get(i).copied()
    }
}

impl Code for BigInt {
    fn put(&self, _: &mut Writer) -> Option<()> {
        None
    }
    fn get(_: &mut Reader) -> Option<Self> {
        None
    }
}

impl Code for PropertyKey {
    fn put(&self, w: &mut Writer) -> Option<()> {
        match self {
            PropertyKey::Index(i) => {
                w.u8(0);
                w.u32(*i);
            }
            PropertyKey::String(s) => {
                w.u8(1);
                s.put(w)?;
            }
            PropertyKey::Symbol(_) => return None,
        }
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        match r.u8()? {
            0 => Some(PropertyKey::Index(r.u32()?)),
            1 => Some(PropertyKey::String(JsString::get(r)?)),
            _ => None,
        }
    }
}

impl<T: Code> Code for Option<T> {
    fn put(&self, w: &mut Writer) -> Option<()> {
        match self {
            None => w.u8(0),
            Some(v) => {
                w.u8(1);
                v.put(w)?;
            }
        }
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        match r.u8()? {
            0 => Some(None),
            1 => Some(Some(T::get(r)?)),
            _ => None,
        }
    }
}

impl<T: Code> Code for Vec<T> {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.len(self.len())?;
        for v in self {
            v.put(w)?;
        }
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        let n = r.len()?;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(T::get(r)?);
        }
        Some(out)
    }
}

/// Writes a slice as a `Vec` or boxed slice reads it back.
fn put_slice<T: Code>(items: &[T], w: &mut Writer) -> Option<()> {
    w.len(items.len())?;
    for v in items {
        v.put(w)?;
    }
    Some(())
}

impl<T: Code> Code for Box<[T]> {
    fn put(&self, w: &mut Writer) -> Option<()> {
        put_slice(self, w)
    }
    fn get(r: &mut Reader) -> Option<Self> {
        Vec::get(r).map(Vec::into_boxed_slice)
    }
}

impl<A: Code, B: Code> Code for (A, B) {
    fn put(&self, w: &mut Writer) -> Option<()> {
        self.0.put(w)?;
        self.1.put(w)
    }
    fn get(r: &mut Reader) -> Option<Self> {
        Some((A::get(r)?, B::get(r)?))
    }
}

impl Code for String {
    fn put(&self, w: &mut Writer) -> Option<()> {
        w.len(self.len())?;
        w.buf.extend_from_slice(self.as_bytes());
        Some(())
    }
    fn get(r: &mut Reader) -> Option<Self> {
        let n = r.len()?;
        let s = std::str::from_utf8(r.buf.get(r.pos..r.pos + n)?).ok()?;
        r.pos += n;
        Some(s.to_string())
    }
}

/// Fieldless enums, as their position in the list.
macro_rules! unit_enum_code {
    ($ty:ident { $($v:ident),* $(,)? }) => {
        impl Code for $ty {
            fn put(&self, w: &mut Writer) -> Option<()> {
                const ALL: &[$ty] = &[$($ty::$v),*];
                w.u8(ALL.iter().position(|v| v == self)? as u8);
                Some(())
            }
            fn get(r: &mut Reader) -> Option<Self> {
                const ALL: &[$ty] = &[$($ty::$v),*];
                ALL.get(r.u8()? as usize).copied()
            }
        }
    };
}

unit_enum_code!(BinOp {
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
});
unit_enum_code!(ErrorType {
    Type,
    Reference,
    Syntax
});
unit_enum_code!(MethodKind {
    Method,
    Getter,
    Setter
});
unit_enum_code!(CodeKind {
    Script,
    Module,
    Eval,
    Function
});
unit_enum_code!(FunctionFlavor {
    Normal,
    Arrow,
    Method,
    Getter,
    Setter,
    ClassConstructor,
    DerivedConstructor,
    ClassFieldInit,
});
unit_enum_code!(LocalKind {
    Param,
    Var,
    Let,
    Const
});
unit_enum_code!(ThisMode {
    Lexical,
    Strict,
    Global
});

/// Structs, field by field.
macro_rules! struct_code {
    ($ty:ident { $($f:ident),* $(,)? }) => {
        impl Code for $ty {
            fn put(&self, w: &mut Writer) -> Option<()> {
                $( self.$f.put(w)?; )*
                Some(())
            }
            fn get(r: &mut Reader) -> Option<Self> {
                Some($ty { $( $f: Code::get(r)?, )* })
            }
        }
    };
}

struct_code!(MethodDef { kind, enumerable });
struct_code!(LocalInfo { name, kind });
struct_code!(ClassInfo { constructor, class });
struct_code!(FieldDef {
    is_static,
    private,
    static_block,
    auto_accessor
});
struct_code!(PrivateMethodDef {
    name,
    kind,
    is_static
});
struct_code!(ScopeInfo {
    lexical,
    functions,
    is_catch
});
struct_code!(Declarations {
    var_names,
    functions,
    lexical,
    annex_b_functions
});
struct_code!(Params {
    names,
    simple,
    has_expressions,
    has_duplicates
});
struct_code!(TemplateInfo { cooked, raw, site });
struct_code!(ClassStatic {
    name,
    private_names,
    span
});

/// The instructions, each as a tag (its position in this list) and its
/// operands. Adding an instruction to [`Op`] without adding it here fails
/// to compile.
macro_rules! op_code {
    ($($tag:literal $name:ident $( ( $($f:ident : $t:ty),* ) )? ,)*) => {
        impl Code for Op {
            fn put(&self, w: &mut Writer) -> Option<()> {
                match *self {
                    $( Op::$name $( ( $($f),* ) )? => {
                        w.u8($tag);
                        $( $( $f.put(w)?; )* )?
                    } )*
                }
                Some(())
            }
            fn get(r: &mut Reader) -> Option<Self> {
                Some(match r.u8()? {
                    $( $tag => Op::$name $( ( $( <$t as Code>::get(r)? ),* ) )?, )*
                    _ => return None,
                })
            }
        }
    };
}

op_code! {
    0 Undefined,
    1 Null,
    2 True,
    3 False,
    4 Number(a: f64),
    5 String(a: JsString),
    6 BigInt(a: BigInt),
    7 Pop,
    8 Dup,
    9 Dup2,
    10 Swap,
    11 RotRight3,
    12 RotRight4,
    13 RotRight5,
    14 Pick(a: u32),
    15 SetCompletion,
    16 GetCompletion,
    17 GetName(a: JsString, b: u32),
    18 TypeofName(a: JsString, b: u32),
    19 GetNameForCall(a: JsString, b: u32),
    20 ResolveName(a: JsString, b: u32),
    21 GetRef,
    22 PutRef,
    23 DropRef,
    24 InitName(a: JsString),
    25 DeleteName(a: JsString),
    26 AnnexBFunction(a: JsString),
    27 PushScope(a: u32),
    28 PushEmptyScope,
    29 PopScope,
    30 CopyScope,
    31 PushWith,
    32 GetProp(a: PropertyKey, b: u32),
    33 GetElem,
    34 GetElemForUpdate,
    35 GetPropForCall(a: PropertyKey, b: u32),
    36 GetElemForCall,
    37 SetProp(a: PropertyKey, b: u32),
    38 SetElem,
    39 DeleteProp(a: PropertyKey),
    40 DeleteElem,
    41 ToPropertyKey,
    42 RequireObjectCoercible,
    43 GetSuperProp(a: PropertyKey),
    44 GetSuperPropForCall(a: PropertyKey),
    45 SuperBase,
    46 GetSuperElem,
    47 GetSuperElemForCall,
    48 GetSuperElemForUpdate,
    49 SetSuperElem,
    50 GetPrivate(a: JsString),
    51 SetPrivate(a: JsString),
    52 GetPrivateForCall(a: JsString),
    53 PrivateIn(a: JsString),
    54 Binary(a: BinOp),
    55 Neg,
    56 Plus,
    57 BitNot,
    58 Not,
    59 Typeof,
    60 ToNumeric,
    61 Inc,
    62 Dec,
    63 ToString,
    64 Jump(a: Label),
    65 JumpIfFalse(a: Label),
    66 JumpIfTrue(a: Label),
    67 JumpIfFalseKeep(a: Label),
    68 JumpIfTrueKeep(a: Label),
    69 JumpIfNullishKeep(a: Label),
    70 JumpIfNotNullishKeep(a: Label),
    71 JumpIfNotUndefined(a: Label),
    72 Throw,
    73 ThrowError(a: ErrorType, b: u32),
    74 PushHandler(a: Label),
    75 PopHandler,
    76 EndFinally(a: u32),
    77 AddDisposable(a: bool),
    78 DisposeStart,
    79 DisposeStep(a: Label),
    80 DisposeAwaitThrew,
    81 DisposeEnd,
    82 Closure(a: u32),
    83 ClosureNamed(a: u32),
    84 FieldInitializerName,
    85 Call(a: u32),
    86 CallSpread,
    87 CallEval(a: u32),
    88 CallEvalSpread,
    89 New(a: u32),
    90 NewSpread,
    91 GetSuperConstructor,
    92 SuperCall(a: u32),
    93 SuperCallSpread,
    94 Return,
    95 This,
    96 NewTarget,
    97 ImportMeta,
    98 ImportCall,
    99 GetArg(a: u32),
    100 GetRestArgs(a: u32),
    101 InstantiateFunctionBody,
    102 NewObject,
    103 NewArray(a: u32),
    104 DefineField(a: PropertyKey),
    105 DefineFieldComputed,
    106 ArrayPush,
    107 ArrayHole,
    108 ArraySpread,
    109 CopyDataProperties,
    110 CopyDataPropertiesExcluding(a: u32),
    111 SetPrototypeLiteral,
    112 DefineMethod(a: MethodDef),
    113 GetTemplateObject(a: u32),
    114 RegExp(a: JsString, b: JsString),
    115 GetIterator,
    116 GetAsyncIterator,
    117 IteratorNext(a: Label),
    118 IteratorNextOrUndefined,
    119 IteratorSkip,
    120 IteratorRest,
    121 PopIterator,
    122 IteratorClose,
    123 IteratorCloseForThrow,
    124 IteratorCallNext,
    125 IteratorResultUnwrap(a: Label),
    126 AsyncIteratorClose(a: Label),
    127 AsyncIteratorCloseForThrow(a: Label),
    128 IteratorCheckObject,
    129 ForInStart,
    130 ForInNext(a: Label),
    131 GeneratorStart,
    132 Yield(a: Label),
    133 YieldDelegate(a: Label),
    134 Await,
    135 SetDelegateKind(a: u8),
    136 JumpIfDelegateKind(a: u8, b: Label),
    137 AsyncDelegateCall(a: Label),
    138 AsyncDelegateReturn(a: Label),
    139 AsyncDelegateResult(a: Label),
    140 DelegateYield,
    141 ClassHeritage(a: bool),
    142 CreateClass(a: ClassInfo),
    143 PushPrivateScope(a: u32),
    144 PopPrivateScope,
    145 DefineClassField(a: FieldDef),
    146 DefinePrivateMethod(a: PrivateMethodDef),
    147 FinishClass,
    148 SuperCallForward,
    149 NotImplemented(a: u32),
    150 Debugger,
    151 Nop,
    152 GetLocal(a: u32),
    153 ResolveLocal(a: u32),
    154 InitLocal(a: u32),
    155 ClearLocals(a: u32, b: u32),
}

/// The surroundings every function of a decoded tree shares.
struct Context<'a> {
    source: &'a Arc<crate::source::SourceText>,
    script: u32,
    constants: &'a Arc<CodeConstants>,
}

fn put_code(code: &FunctionCode, w: &mut Writer) -> Option<()> {
    code.kind.put(w)?;
    code.flavor.put(w)?;
    code.is_async.put(w)?;
    code.is_generator.put(w)?;
    code.strict.put(w)?;
    code.this_mode.put(w)?;
    code.name.put(w)?;
    code.length.put(w)?;
    code.params.put(w)?;
    code.decls().put(w)?;
    code.uses_arguments.put(w)?;
    code.has_direct_eval.put(w)?;
    code.binds_own_name.put(w)?;
    code.envs_die_with_call.put(w)?;
    code.locals.as_deref().map(<[LocalInfo]>::to_vec).put(w)?;
    code.skips_function_env.put(w)?;
    code.ops.put(w)?;
    code.positions.put(w)?;
    w.len(code.functions.len())?;
    for f in &code.functions {
        put_code(f, w)?;
    }
    put_slice(code.scopes(), w)?;
    put_slice(code.templates(), w)?;
    put_slice(code.classes(), w)?;
    put_slice(code.messages(), w)?;
    put_slice(code.finally_targets(), w)?;
    put_slice(code.strict_ranges(), w)?;
    code.span.put(w)?;
    w.len(code.name_caches.len())?;
    w.len(code.prop_caches.len())?;
    match &code.lazy {
        None => w.u8(0),
        Some(lazy) => {
            w.u8(1);
            lazy.function.outer_strict.put(w)?;
            lazy.function.module.put(w)?;
            lazy.dynamic_scopes.put(w)?;
            lazy.is_expression.put(w)?;
        }
    }
    Some(())
}

fn get_code(r: &mut Reader, cx: &Context) -> Option<FunctionCode> {
    let kind = Code::get(r)?;
    let flavor = Code::get(r)?;
    let is_async = Code::get(r)?;
    let is_generator = Code::get(r)?;
    let strict = Code::get(r)?;
    let this_mode = Code::get(r)?;
    let name = Code::get(r)?;
    let length = Code::get(r)?;
    let params = Code::get(r)?;
    let decls: Declarations = Code::get(r)?;
    let uses_arguments = Code::get(r)?;
    let has_direct_eval = Code::get(r)?;
    let binds_own_name = Code::get(r)?;
    let envs_die_with_call = Code::get(r)?;
    let locals: Option<Vec<LocalInfo>> = Code::get(r)?;
    let skips_function_env = Code::get(r)?;
    let ops = Code::get(r)?;
    let positions = Code::get(r)?;
    let n = r.len()?;
    let mut functions = Vec::with_capacity(n);
    for _ in 0..n {
        functions.push(Arc::new(get_code(r, cx)?));
    }
    let tables = CodeTables {
        scopes: Code::get(r)?,
        templates: Code::get(r)?,
        classes: Code::get(r)?,
        messages: Code::get(r)?,
        finally_targets: Code::get(r)?,
        strict_ranges: Code::get(r)?,
    };
    let span = Code::get(r)?;
    let name_caches = (0..r.u32()?)
        .map(|_| std::sync::atomic::AtomicU64::new(0))
        .collect();
    let prop_caches = (0..r.u32()?).map(|_| PropCache::default()).collect();
    let lazy = match r.u8()? {
        0 => None,
        1 => Some(Box::new(LazyCode {
            function: kahvi_parser::ast::LazyFunction {
                outer_strict: Code::get(r)?,
                module: Code::get(r)?,
            },
            dynamic_scopes: Code::get(r)?,
            is_expression: Code::get(r)?,
            compiled: std::sync::OnceLock::new(),
        })),
        _ => return None,
    };
    Some(FunctionCode {
        kind,
        flavor,
        is_async,
        is_generator,
        strict,
        this_mode,
        name,
        length,
        params,
        decls: (decls.var_names.len()
            + decls.functions.len()
            + decls.lexical.len()
            + decls.annex_b_functions.len()
            > 0)
        .then(|| Box::new(decls)),
        uses_arguments,
        has_direct_eval,
        binds_own_name,
        envs_die_with_call,
        locals: locals.map(Vec::into_boxed_slice),
        skips_function_env,
        ops,
        positions,
        functions: functions.into(),
        tables: tables.boxed(),
        source: cx.source.clone(),
        span,
        script: cx.script,
        constants: cx.constants.clone(),
        name_caches,
        prop_caches,
        lazy,
    })
}

/// `code` as bytes, or `None` if it holds something the codec cannot
/// represent.
pub(crate) fn encode(agent: &Agent, code: &FunctionCode) -> Option<Vec<u8>> {
    let mut w = Writer {
        buf: Vec::new(),
        strings: Vec::new(),
        index: HashMap::new(),
    };
    put_code(code, &mut w)?;
    let body = std::mem::take(&mut w.buf);
    let strings = std::mem::take(&mut w.strings);
    w.len(strings.len())?;
    for s in strings {
        let units = agent.str(s);
        w.len(units.len())?;
        for &u in units {
            w.buf.extend_from_slice(&u.to_le_bytes());
        }
    }
    w.buf.extend_from_slice(&body);
    Some(w.buf)
}

/// The code `bytes` encodes, as the compiled code of lazy function `stub`
/// (whose source, script and constants it takes), or `None` if `bytes` is
/// not a valid encoding. Run inside a compilation unit (`pinned`), which
/// keeps the strings it interns alive.
pub(crate) fn decode(
    agent: &mut Agent,
    bytes: &[u8],
    stub: &FunctionCode,
    constants: &Arc<CodeConstants>,
) -> Option<FunctionCode> {
    let mut r = Reader {
        buf: bytes,
        pos: 0,
        strings: Vec::new(),
    };
    let n = r.len()?;
    let mut units = Vec::new();
    let mut strings = Vec::with_capacity(n);
    for _ in 0..n {
        let len = r.len()?;
        let raw = r.buf.get(r.pos..r.pos + 2 * len)?;
        r.pos += 2 * len;
        units.clear();
        units.extend(
            raw.chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]])),
        );
        strings.push(agent.intern(&units));
    }
    r.strings = strings;
    let cx = Context {
        source: &stub.source,
        script: stub.script,
        constants,
    };
    let code = get_code(&mut r, &cx)?;
    (r.pos == bytes.len() && code.span == stub.span && code.lazy.is_none()).then_some(code)
}
