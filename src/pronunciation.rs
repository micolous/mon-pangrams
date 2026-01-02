//! Load in pronunciation data

use eyre::{bail, Result};
use std::io::BufRead;

/// All the phonemes that can appear in a Pokémon's name.
pub const MON_PHONEMES: [char; 38] = [
    'b', 'd', 'f', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'p', 's', 't', 'u', 'v', 'w', 'z', 'Ø', 'ä',
    'æ', 'ï', 'ð', 'õ', 'ö', 'ŋ', 'ɑ', 'ə', 'ɚ', 'ɛ', 'ɡ', 'ɪ', 'ɹ', 'ʃ', 'ʊ', 'ʒ', 'ʤ', 'ʧ', 'θ',
];

pub fn phoneme_index(phoneme: char) -> Option<usize> {
    MON_PHONEMES.iter().position(|p| *p == phoneme)
}

/// Set that holds usize values from 0 to 63 in the bits of a `u64` value. Overflows of the inserted
/// index may panic.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct BitSet {
    bits: u64,
}

impl BitSet {
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn insert(&mut self, index: usize) {
        self.bits |= 1 << index;
    }

    pub fn contains(&self, index: usize) -> bool {
        self.bits & (1 << index) != 0
    }

    pub fn is_subset_of(self, other: Self) -> bool {
        (self & other) == self
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + use<'_> {
        (0..64usize)
            .into_iter()
            .filter(|n| self.bits & (1 << *n) != 0)
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut res = BitSet::default();
        for index in iter {
            res.insert(index);
        }
        res
    }
}

// Implement set union
impl std::ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl std::ops::BitOr for BitSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self {
        self |= rhs;
        self
    }
}

// Implement set intersection
impl std::ops::BitAndAssign for BitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl std::ops::BitAnd for BitSet {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}

// Implement set subtraction
impl std::ops::SubAssign for BitSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits &= !rhs.bits;
    }
}

impl std::ops::Sub for BitSet {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

/// Reader for pronunciation files
pub struct PronunciationReader<R> {
    f: std::io::Lines<R>,
    next_id: u16,
}

/// A Pokémon's pronunciation entry
#[derive(Clone, Debug, PartialEq)]
pub struct Pokémon {
    /// The name
    pub name: String,
    /// Its pronunciation, in IPA
    pub ipa: String,
    /// The mask of phonemes that appear in this Pokémon's IPA
    pub phonemes_mask: BitSet,
    /// ID of the pokemon; Actually a line number
    pub id: u16,
}

impl<R> PronunciationReader<R>
where
    R: BufRead,
{
    /// Read pronunciation data file
    pub fn new(f: R) -> Self {
        Self {
            f: f.lines(),
            next_id: 0,
        }
    }
}

impl<R> Iterator for PronunciationReader<R>
where
    R: BufRead,
{
    type Item = Result<Pokémon>;

    /// Read the next [Pokemon] in the file.
    fn next(&mut self) -> Option<Result<Pokémon>> {
        while let Some(line_read) = self.f.next() {
            let line = match line_read {
                Ok(line) => line,
                Err(e) => return Some(Err(e.into())),
            };
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let Some((name, ipa)) = line.split_once(" /") else {
                // actually an error, but we'll just skip this
                eprintln!("Unexpected entry formatting: {line:?}");
                continue;
            };

            let id = self.next_id;
            self.next_id += 1;
            return Some(Pokémon::new(name, ipa, id));
        }

        // EOF
        None
    }
}

impl Pokémon {
    fn new(name: &str, ipa: &str, id: u16) -> Result<Self> {
        let name = name.trim();
        let ipa = clean_pronunciation(ipa)?;

        Ok(Pokémon {
            name: name.to_string(),
            phonemes_mask: ipa.chars().flat_map(phoneme_index).collect(),
            ipa,
            id,
        })
    }
}

/// Clean up pronunciations
pub fn clean_pronunciation(mut i: &str) -> Result<String> {
    // Trim whitespace characters
    i = i.trim_matches(|c: char| c == '/' || c.is_ascii_whitespace());

    // Remove Nidoran gender (appears in brackets)
    if let Some((j, _)) = i.split_once('(') {
        i = j.trim();
    }

    let mut o = i.to_string();

    // Hydreigon; probably a typo because the combining character at the start, and that phoneme
    // isn't used in US English.
    o = o.replace("\u{329}h", "h");

    // Use single codepoints for digraphs
    o = o.replace("dʒ", "ʤ");
    o = o.replace("tʃ", "ʧ");
    o = o.replace("oʊ", "ö");
    o = o.replace("aɪ", "ï");
    o = o.replace("eɪ", "ä");
    o = o.replace("ɔɪ", "õ");
    o = o.replace("aʊ", "Ø");

    o = o.replace("ɑɪ", "ï"); //Some names (arcanine, omanyte) used the wrong diphthong
    o = o.replace("r", "ɹ"); //English has no r, only ɹ
    o = o.replace("g", "ɡ"); //Use ɡ (the actual phoneme), not g
    o = o.replace("a", "ɑ"); //Use ɑ (the actual phoneme), not a

    o = o.replace("ɾ", "d"); //GA treats ɾ as d
    o = o.replace("ʌ", "ə"); //GA treats ʌ as ə
    o = o.replace("ɜ", "ə"); //GA treats ɜ as ə
    o = o.replace("ɝ", "ɚ"); //GA treats ɜr as ər
    o = o.replace("ɔ", "ɑ"); //GA treats ɔ as ɑ

    // Remove things that aren't phonemes
    let non_phoeneme = " ːˈˌʔ";
    o = o.replace(|p| non_phoeneme.contains(p), "");

    // If we hit an error here, then this function or MON_PHONEMES needs updating.
    if !o.chars().all(|c| MON_PHONEMES.contains(&c)) {
        bail!("unexpected character after cleaning {o:?}: {i:?}");
    }

    Ok(o)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cleanup() {
        assert_eq!(
            "nidöɹæn",
            clean_pronunciation("/ˈniːdoʊɹæn (ˈfiːmeɪl)/").unwrap()
        );
        assert_eq!("ʤɪɡlipəf", clean_pronunciation("/ˈdʒɪɡliːpʌf/").unwrap());
        assert_eq!(
            "hïdɹïɡən",
            clean_pronunciation("/\u{329}haɪˈdraɪɡən/").unwrap()
        );
    }
}
