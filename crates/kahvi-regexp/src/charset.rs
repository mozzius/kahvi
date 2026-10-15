//! CharSets (§22.2.2): sets of characters and, with the `v` flag, strings;
//! the character class escapes, property escapes and Canonicalize.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use kahvi_unicode as uc;

/// The parts of the RegExp Record that matching and CharSet compilation read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Rer {
    pub ignore_case: bool,
    pub multiline: bool,
    pub dot_all: bool,
    pub unicode: bool,
    pub unicode_sets: bool,
}

impl Rer {
    /// HasEitherUnicodeFlag(rer)
    pub fn either_unicode(self) -> bool {
        self.unicode || self.unicode_sets
    }
}

/// A CharSet: single characters as sorted disjoint ranges, plus the strings
/// of zero or two or more characters (only with the `v` flag).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CharSet {
    pub ranges: Vec<(u32, u32)>,
    pub strings: BTreeSet<Vec<u32>>,
    /// MayContainStrings of the syntax this set came from (a static property:
    /// `[\q{ab}&&\q{cd}]` may contain strings although it has none).
    pub may_contain_strings: bool,
}

impl CharSet {
    pub fn empty() -> CharSet {
        CharSet::default()
    }

    pub fn from_ranges(ranges: Vec<(u32, u32)>) -> CharSet {
        CharSet {
            ranges: uc::normalize(ranges),
            strings: BTreeSet::new(),
            may_contain_strings: false,
        }
    }

    pub fn char(c: u32) -> CharSet {
        CharSet::from_ranges(vec![(c, c)])
    }

    /// A string element; a one-character string is a character.
    pub fn string(s: Vec<u32>) -> CharSet {
        if s.len() == 1 {
            return CharSet::char(s[0]);
        }
        let mut set = CharSet::empty();
        set.strings.insert(s);
        set.may_contain_strings = true;
        set
    }

    pub fn union(mut self, other: &CharSet) -> CharSet {
        self.ranges.extend_from_slice(&other.ranges);
        self.ranges = uc::normalize(std::mem::take(&mut self.ranges));
        self.strings.extend(other.strings.iter().cloned());
        self.may_contain_strings |= other.may_contain_strings;
        self
    }

    pub fn intersection(&self, other: &CharSet) -> CharSet {
        let not_other = uc::subtract(&self.ranges, &other.ranges);
        CharSet {
            ranges: uc::subtract(&self.ranges, &not_other),
            strings: self.strings.intersection(&other.strings).cloned().collect(),
            may_contain_strings: self.may_contain_strings && other.may_contain_strings,
        }
    }

    pub fn subtract(&self, other: &CharSet) -> CharSet {
        CharSet {
            ranges: uc::subtract(&self.ranges, &other.ranges),
            strings: self.strings.difference(&other.strings).cloned().collect(),
            may_contain_strings: self.may_contain_strings,
        }
    }

    /// MayContainStrings
    pub fn has_strings(&self) -> bool {
        self.may_contain_strings
    }
}

/// CharacterRange(A, B) for two single characters.
pub(crate) fn character_range(lo: u32, hi: u32) -> CharSet {
    CharSet::from_ranges(vec![(lo, hi)])
}

/// AllCharacters(rer) (§22.2.2.9.1)
pub(crate) fn all_characters(rer: Rer) -> CharSet {
    if rer.unicode_sets && rer.ignore_case {
        // 1. ... all Unicode code points c that do not have a Simple Case Folding mapping (that is, scf(c)=c).
        let changed: Vec<(u32, u32)> = uc::simple_case_foldings().map(|(c, _)| (c, c)).collect();
        CharSet::from_ranges(uc::subtract(
            &[(0, uc::MAX_CODE_POINT)],
            &uc::normalize(changed),
        ))
    } else if rer.either_unicode() {
        // 2. Else if HasEitherUnicodeFlag(rer) is true, all code points.
        character_range(0, uc::MAX_CODE_POINT)
    } else {
        // 3. Else, all code units.
        character_range(0, 0xFFFF)
    }
}

/// CharacterComplement(rer, S) (§22.2.2.9.2): the characters of
/// AllCharacters(rer) not in S.
pub(crate) fn character_complement(rer: Rer, s: &CharSet) -> CharSet {
    let all = all_characters(rer);
    CharSet::from_ranges(uc::subtract(&all.ranges, &s.ranges))
}

/// MaybeSimpleCaseFolding(rer, A) (§22.2.2.9.3)
pub(crate) fn maybe_simple_case_folding(rer: Rer, a: CharSet) -> CharSet {
    // 1. If rer.[[UnicodeSets]] is false or rer.[[IgnoreCase]] is false, return A.
    if !rer.unicode_sets || !rer.ignore_case {
        return a;
    }
    // 2-4. B is the CharSet of each element of A with scf applied to each of its code points.
    let ranges = canonicalize_ranges(&a.ranges, &FOLD);
    let strings = a
        .strings
        .iter()
        .map(|s| s.iter().map(|&c| uc::simple_case_fold(c)).collect())
        .collect();
    CharSet {
        ranges,
        strings,
        may_contain_strings: a.may_contain_strings,
    }
}

// ---- Canonicalize ----------------------------------------------------------

/// Which Canonicalize applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fold {
    /// rer.[[IgnoreCase]] is false: the identity.
    None,
    /// HasEitherUnicodeFlag(rer) and IgnoreCase: simple case folding.
    Unicode,
    /// IgnoreCase without either Unicode flag: the toUppercase-based mapping.
    Legacy,
}

impl Fold {
    pub fn of(rer: Rer) -> Fold {
        if !rer.ignore_case {
            Fold::None
        } else if rer.either_unicode() {
            Fold::Unicode
        } else {
            Fold::Legacy
        }
    }

    /// Canonicalize(rer, ch) (§22.2.2.7.3)
    pub fn canonicalize(self, ch: u32) -> u32 {
        match self {
            Fold::None => ch,
            // 1. If HasEitherUnicodeFlag(rer) and rer.[[IgnoreCase]] are true, then
            //    a. If the file CaseFolding.txt provides a simple or common case folding mapping for ch,
            //       return the result of applying that mapping to ch.
            //    b. Return ch.
            Fold::Unicode => uc::simple_case_fold(ch),
            Fold::Legacy => legacy_canonicalize(ch),
        }
    }
}

const FOLD: Fold = Fold::Unicode;

fn legacy_canonicalize(ch: u32) -> u32 {
    // 3. Assert: ch is a UTF-16 code unit.
    // 4. Let cp be the code point whose numeric value is the numeric value of ch.
    // 5. Let u be toUppercase(« cp »), according to the Unicode Default Case Conversion algorithm.
    // 6. Let uStr be CodePointsToString(u).
    // 7. If the length of uStr ≠ 1, return ch.
    let Some(u) = uc::to_uppercase(ch) else {
        return ch;
    };
    if u.len() != 1 || u[0] > 0xFFFF {
        return ch;
    }
    // 8. Let cu be uStr's single code unit element.
    let cu = u[0];
    // 9. If the numeric value of ch ≥ 128 and the numeric value of cu < 128, return ch.
    if ch >= 128 && cu < 128 {
        return ch;
    }
    // 10. Return cu.
    cu
}

/// The characters whose canonical form differs from themselves, per fold kind.
fn changed_by(fold: Fold) -> &'static [(u32, u32)] {
    static UNICODE: OnceLock<Vec<(u32, u32)>> = OnceLock::new();
    static LEGACY: OnceLock<Vec<(u32, u32)>> = OnceLock::new();
    match fold {
        Fold::None => &[],
        Fold::Unicode => UNICODE.get_or_init(|| uc::simple_case_foldings().collect()),
        Fold::Legacy => LEGACY.get_or_init(|| {
            uc::uppercase_mappings()
                .filter(|&(c, _)| c <= 0xFFFF)
                .map(|(c, _)| (c, legacy_canonicalize(c)))
                .filter(|&(c, cu)| c != cu)
                .collect()
        }),
    }
}

/// { Canonicalize(c) | c ∈ ranges }, without visiting every character: only
/// characters that Canonicalize changes move.
pub(crate) fn canonicalize_ranges(ranges: &[(u32, u32)], fold: &Fold) -> Vec<(u32, u32)> {
    let changed = changed_by(*fold);
    if changed.is_empty() {
        return ranges.to_vec();
    }
    let moved: Vec<(u32, u32)> = changed.iter().map(|&(c, _)| (c, c)).collect();
    let mut out = uc::subtract(ranges, &moved);
    for &(c, to) in changed {
        if uc::in_ranges(ranges, c) {
            out.push((to, to));
        }
    }
    uc::normalize(out)
}

// ---- Character class escapes -------------------------------------------------

pub(crate) fn digits() -> CharSet {
    character_range('0' as u32, '9' as u32)
}

/// The WhiteSpace and LineTerminator code points (\s).
pub(crate) fn white_space() -> CharSet {
    let mut ranges = vec![
        (0x09, 0x0D), // TAB, LF, VT, FF, CR
        (0x20, 0x20),
        (0xA0, 0xA0),
        (0x2028, 0x2029),
        (0xFEFF, 0xFEFF),
    ];
    ranges.extend(uc::general_category_ranges(
        uc::general_category_mask("Zs").expect("Zs is a General_Category value"),
    ));
    CharSet::from_ranges(ranges)
}

/// The line terminators.
pub(crate) fn is_line_terminator(c: u32) -> bool {
    matches!(c, 0x0A | 0x0D | 0x2028 | 0x2029)
}

/// WordCharacters(rer) (§22.2.2.9.4)
pub(crate) fn word_characters(rer: Rer) -> CharSet {
    // 1. Let basicWordChars be the CharSet containing every character in
    //    the ASCII word characters.
    let basic = vec![
        ('a' as u32, 'z' as u32),
        ('A' as u32, 'Z' as u32),
        ('0' as u32, '9' as u32),
        ('_' as u32, '_' as u32),
    ];
    // 2. Let extraWordChars be the CharSet containing all characters c such that c is not in
    //    basicWordChars but Canonicalize(rer, c) is in basicWordChars.
    // 3. Assert: extraWordChars is empty unless HasEitherUnicodeFlag(rer) is true and rer.[[IgnoreCase]] is true.
    let fold = Fold::of(rer);
    let basic = uc::normalize(basic);
    let mut all = basic.clone();
    for &(c, to) in changed_by(fold) {
        if !uc::in_ranges(&basic, c) && uc::in_ranges(&basic, to) {
            all.push((c, c));
        }
    }
    // 4. Return the union of basicWordChars and extraWordChars.
    CharSet::from_ranges(all)
}

/// Whether a property escape is valid, and its CharSet if so.
pub(crate) enum PropertyError {
    Unknown,
    StringsWithoutV,
}

/// CompileToCharSet of a UnicodePropertyValueExpression (§22.2.2.9), before MaybeSimpleCaseFolding.
pub(crate) fn property_set(
    name: &str,
    value: Option<&str>,
    sets_mode: bool,
) -> Result<CharSet, PropertyError> {
    match value {
        Some(value) => {
            // UnicodePropertyName `=` UnicodePropertyValue
            let ranges = match name {
                "General_Category" | "gc" => uc::general_category_ranges(
                    uc::general_category_mask(value).ok_or(PropertyError::Unknown)?,
                ),
                "Script" | "sc" => {
                    uc::script_ranges(uc::script_by_name(value).ok_or(PropertyError::Unknown)?)
                }
                "Script_Extensions" | "scx" => uc::script_extensions_ranges(
                    uc::script_by_name(value).ok_or(PropertyError::Unknown)?,
                ),
                _ => return Err(PropertyError::Unknown),
            };
            Ok(CharSet::from_ranges(ranges))
        }
        None => {
            // LoneUnicodePropertyNameOrValue: a General_Category value, a binary property,
            // or (with v) a binary property of strings.
            if let Some(mask) = uc::general_category_mask(name) {
                return Ok(CharSet::from_ranges(uc::general_category_ranges(mask)));
            }
            if let Some(ranges) = uc::binary_property(name) {
                return Ok(CharSet::from_ranges(ranges.to_vec()));
            }
            if let Some((cps, strings)) = uc::string_property(name) {
                if !sets_mode {
                    return Err(PropertyError::StringsWithoutV);
                }
                let mut set = CharSet::from_ranges(cps.to_vec());
                // MayContainStrings: a binary property of strings.
                set.may_contain_strings = true;
                for s in strings {
                    set.strings.insert(s.to_vec());
                }
                return Ok(set);
            }
            Err(PropertyError::Unknown)
        }
    }
}
