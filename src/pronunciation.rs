//! Load in pronunciation data

use crate::set::BitSet;
use eyre::{Result, bail};
use std::io::BufRead;

/// All the phonemes that can appear in a Pokémon's name.
pub const MON_PHONEMES: [char; 38] = [
    'b', 'd', 'f', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'p', 's', 't', 'u', 'v', 'w', 'z', 'Ø', 'ä',
    'æ', 'ï', 'ð', 'õ', 'ö', 'ŋ', 'ɑ', 'ə', 'ɚ', 'ɛ', 'ɡ', 'ɪ', 'ɹ', 'ʃ', 'ʊ', 'ʒ', 'ʤ', 'ʧ', 'θ',
];

pub fn phoneme_index(phoneme: char) -> Option<usize> {
    MON_PHONEMES.iter().position(|p| *p == phoneme)
}

/// Reader for pronunciation files
pub struct PronunciationReader<R> {
    lines: std::io::Lines<R>,
}

/// A Pokémon's pronunciation entry
#[derive(Clone, Debug)]
pub struct Pokémon {
    /// The mask of phonemes that appear in this Pokémon's IPA
    pub phoneme_set: BitSet,
    text: Box<PkmnText>,
}

impl Pokémon {
    pub fn name(&self) -> &str {
        &self.text.name
    }

    pub fn ipa(&self) -> &str {
        &self.text.ipa
    }
}

#[derive(Clone, Debug)]
struct PkmnText {
    /// The name
    name: String,
    /// Its pronunciation, in IPA
    ipa: String,
}

impl<R> PronunciationReader<R>
where
    R: BufRead,
{
    /// Read pronunciation data file
    pub fn new(f: R) -> Self {
        Self { lines: f.lines() }
    }
}

impl<R> Iterator for PronunciationReader<R>
where
    R: BufRead,
{
    type Item = Result<Pokémon>;

    /// Read the next [Pokemon] in the file.
    fn next(&mut self) -> Option<Result<Pokémon>> {
        for line_read in self.lines.by_ref() {
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

            return Some(Pokémon::new(name, ipa));
        }

        // EOF
        None
    }
}

impl Pokémon {
    fn new(name: &str, ipa: &str) -> Result<Self> {
        let name = name.trim();
        let ipa = clean_pronunciation(ipa)?;

        Ok(Pokémon {
            phoneme_set: ipa.chars().flat_map(phoneme_index).collect(),
            text: PkmnText {
                name: name.to_string(),
                ipa,
            }
            .into(),
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
    if !o.chars().all(|c| phoneme_index(c).is_some()) {
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
