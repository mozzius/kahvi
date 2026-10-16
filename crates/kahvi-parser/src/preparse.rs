//! What a parse learned about the bodies of lazy functions (see
//! `ParseOptions::lazy`), so that a later parse of the same source text can
//! skip them without lexing them again.
//!
//! A lazy function's body is checked for early errors and then dropped. The
//! only things the code around it takes from the body are where it ends
//! and a few facts about it (its strictness, whether it uses `arguments` or
//! `this`, whether it contains a direct eval). A parse records those for
//! every lazy function it checks; given the record of an earlier parse of
//! identical source text (and identical options), the parser jumps from the
//! body's `{` to the end of its `}` instead of parsing it. A body is parsed
//! for real when its function is first called (`parse_lazy_function`).

/// One lazy function body: the byte offsets of its `{` and just past its
/// `}`, and what the body told the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkippedBody {
    pub start: u32,
    pub end: u32,
    pub flags: u8,
}

impl SkippedBody {
    pub const STRICT: u8 = 1;
    pub const HAS_USE_STRICT: u8 = 2;
    pub const USES_ARGUMENTS: u8 = 4;
    pub const USES_THIS: u8 = 8;
    pub const HAS_DIRECT_EVAL: u8 = 16;

    pub fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

/// The lazy function bodies of one source text, sorted by `start`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Preparse {
    bodies: Vec<SkippedBody>,
}

/// The first bytes of a serialized [`Preparse`]. Bump the version whenever
/// what the parser records, or what a body may tell it, changes.
const MAGIC: &[u8; 8] = b"kvpp\x00\x00\x00\x01";

impl Preparse {
    pub(crate) fn record(&mut self, body: SkippedBody) {
        // Bodies end in source order but nest, so keep `start` order.
        let at = self.bodies.partition_point(|b| b.start < body.start);
        if self.bodies.get(at).map(|b| b.start) != Some(body.start) {
            self.bodies.insert(at, body);
        }
    }

    /// The recorded body whose `{` is at byte offset `start` of `src`, if
    /// it is consistent with `src`.
    pub(crate) fn body_at(&self, start: usize, src: &str) -> Option<SkippedBody> {
        let i = self
            .bodies
            .binary_search_by_key(&(start as u32), |b| b.start)
            .ok()?;
        let b = self.bodies[i];
        let bytes = src.as_bytes();
        let end = b.end as usize;
        (end > start + 1 && end <= bytes.len() && bytes[start] == b'{' && bytes[end - 1] == b'}')
            .then_some(b)
    }

    pub fn len(&self) -> usize {
        self.bodies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }

    /// A self-describing byte form, for a cache on disk.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(MAGIC.len() + 4 + self.bodies.len() * 9);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&(self.bodies.len() as u32).to_le_bytes());
        for b in &self.bodies {
            out.extend_from_slice(&b.start.to_le_bytes());
            out.extend_from_slice(&b.end.to_le_bytes());
            out.push(b.flags);
        }
        out
    }

    /// The inverse of [`Preparse::to_bytes`]; `None` for anything else
    /// (another version, a truncated file, unsorted bodies).
    pub fn from_bytes(bytes: &[u8]) -> Option<Preparse> {
        let rest = bytes.strip_prefix(MAGIC.as_slice())?;
        let (count, mut rest) = rest.split_first_chunk::<4>()?;
        let count = u32::from_le_bytes(*count) as usize;
        if rest.len() != count.checked_mul(9)? {
            return None;
        }
        let mut bodies = Vec::with_capacity(count);
        while let Some((chunk, tail)) = rest.split_first_chunk::<9>() {
            let start = u32::from_le_bytes(chunk[0..4].try_into().ok()?);
            let end = u32::from_le_bytes(chunk[4..8].try_into().ok()?);
            if bodies
                .last()
                .is_some_and(|b: &SkippedBody| b.start >= start)
                || end <= start
            {
                return None;
            }
            bodies.push(SkippedBody {
                start,
                end,
                flags: chunk[8],
            });
            rest = tail;
        }
        Some(Preparse { bodies })
    }
}
