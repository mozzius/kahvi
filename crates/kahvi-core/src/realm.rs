//! Realms (ECMA-262 §9.3).

use crate::agent::{Agent, ExecutionContext};
use crate::env::{EnvId, Environment};
use crate::error::JsResult;
use crate::object::{ObjectData, ObjectKind};
use crate::value::JsString;
use crate::value::Object;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RealmId(pub u32);

#[derive(Debug)]
pub struct Realm {
    /// [[Intrinsics]] by spec name, e.g. "%Array.prototype%".
    pub intrinsics: crate::hash::FxHashMap<Box<str>, Object>,
    /// [[GlobalObject]]
    pub global_object: Object,
    /// [[GlobalEnv]]
    pub global_env: EnvId,
    /// [[TemplateMap]]: template site (script id, span start) to its array.
    pub template_map: crate::hash::FxHashMap<(u32, u32), Object>,
    /// The legacy static properties of this realm's %RegExp% (Annex B
    /// proposal "Legacy RegExp features"): its [[RegExpInput]] ...
    /// [[RegExpParen9]] internal slots.
    pub legacy_regexp: LegacyRegExpStatics,
    /// Recent `Agent::intrinsic_in` lookups: (name address, name length,
    /// intrinsic). The intrinsics map holds (and roots) every object here.
    pub(crate) intrinsic_cache: [std::cell::Cell<(usize, usize, Option<Object>)>; INTRINSIC_CACHE],
}

/// The number of entries in `Realm::intrinsic_cache`.
pub(crate) const INTRINSIC_CACHE: usize = 64;

/// The values of %RegExp%'s legacy static slots. `None` is ~empty~.
#[derive(Debug, Clone)]
pub struct LegacyRegExpStatics {
    /// [[RegExpInput]]
    pub input: Option<JsString>,
    /// The other slots, set together by the last match.
    pub last_match: Option<LegacyLastMatch>,
}

/// [[RegExpLastMatch]], [[RegExpLastParen]], [[RegExpLeftContext]],
/// [[RegExpRightContext]] and [[RegExpParen1-9]], kept as the matched string
/// and indices; the getters cut the substrings.
#[derive(Debug, Clone)]
pub struct LegacyLastMatch {
    /// The string the match was in.
    pub matched_in: JsString,
    /// startIndex and endIndex of the match.
    pub start: usize,
    pub end: usize,
    /// [[RegExpLastParen]]
    pub last_paren: JsString,
    /// [[RegExpParen1]] ... [[RegExpParen9]]
    pub parens: [JsString; 9],
}

impl Agent {
    /// InitializeHostDefinedRealm (§9.6): a new realm with intrinsics, a
    /// global object and global environment. Returns the realm; it does not
    /// stay on the context stack.
    pub fn create_realm(&mut self) -> JsResult<RealmId> {
        let id = RealmId(self.realms.len() as u32);
        // CreateIntrinsics: the global object and environment are set up
        // below once %Object.prototype% exists.
        let placeholder = self.alloc(ObjectData::new(None, ObjectKind::Ordinary));
        self.realms.push(Realm {
            intrinsics: Default::default(),
            global_object: placeholder,
            global_env: EnvId(0),
            template_map: Default::default(),
            intrinsic_cache: std::array::from_fn(|_| std::cell::Cell::new((0, 0, None))),
            // The initial value of all these internal slots is the empty String.
            legacy_regexp: LegacyRegExpStatics {
                input: Some(self.names.empty),
                last_match: Some(LegacyLastMatch {
                    matched_in: self.names.empty,
                    start: 0,
                    end: 0,
                    last_paren: self.names.empty,
                    parens: [self.names.empty; 9],
                }),
            },
        });
        self.contexts.push(ExecutionContext {
            function: None,
            realm: id,
            script_or_module: None,
        });
        let result = self.install_realm(id);
        self.contexts.pop();
        result?;
        Ok(id)
    }

    fn install_realm(&mut self, id: RealmId) -> JsResult<()> {
        let installer = self
            .installer
            .expect("no intrinsics installer: create runtimes through the kahvi crate");
        // The installer creates the intrinsics and fills the global object.
        let global = self.ordinary_object_create(None);
        self.realm_mut(id).global_object = global;
        let global_env = Environment::new_global(self, global, global);
        self.realm_mut(id).global_env = global_env;
        installer(self, id)
    }

    /// Registers an intrinsic of `realm` under its spec name.
    pub fn set_intrinsic(&mut self, realm: RealmId, name: &str, o: Object) {
        self.realm_mut(realm).intrinsics.insert(name.into(), o);
    }
}
