//! Load in pronunciation data

/// All the phones that can appear in a Pokémon's name.
pub const MON_PHONES: [char; 40] = [
    'b', 'd', 'f', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'p', 's', 't', 'u', 'v', 'w', 'z', 'Ø', 'ä',
    'æ', 'ï', 'ð', 'õ', 'ö', 'ŋ', 'ɑ', 'ə', 'ɚ', 'ɛ', 'ɡ', 'ɪ', 'ɹ', 'ʃ', 'ʊ', 'ʒ', 'ʔ', 'ʤ', 'ʧ',
    'θ', 'ḥ',
];

/// Reader for pronunciation files
pub struct PronunciationReader<R> {
    f: R,
    next_id: u16,
}

/// A Pokémon's pronunciation entry
#[derive(Clone, Debug, PartialEq)]
pub struct Pokémon {
    /// The name
    pub name: String,
    /// Its pronunciation, in IPA
    pub ipa: String,
    /// The mask of phones that appear in this Pokémon's IPA
    pub phones_mask: u64,
    /// Actually a line number
    pub id: u16,
}

impl<R> PronunciationReader<R>
where
    R: std::io::BufRead,
{
    /// Read pronunciation data file
    pub fn new(f: R) -> Self {
        Self { f, next_id: 0 }
    }

    /// Read the next [Pokemon] in the file.
    pub fn next(&mut self) -> std::io::Result<Option<Pokémon>> {
        let mut buf = String::new();

        while self.f.read_line(&mut buf)? != 0 {
            let line = buf.trim();
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
            return Ok(Some(Pokémon::new(name, ipa, id)));
        }

        // EOF
        Ok(None)
    }

    /// Read all remaining [Pokemon] in the file.
    pub fn into_vec(mut self) -> std::io::Result<Vec<Pokémon>> {
        let mut o = Vec::with_capacity(151);

        while let Some(pokemon) = self.next()? {
            o.push(pokemon);
        }

        Ok(o)
    }
}

impl Pokémon {
    fn new(name: &str, ipa: &str, id: u16) -> Self {
        let name = name.trim();
        let ipa = clean_pronunciation(ipa);

        let mut phones_mask = 0;
        for (pos, &phone) in MON_PHONES.iter().enumerate() {
            if ipa.contains(phone) {
                phones_mask |= 1 << pos;
            }
        }

        Pokémon {
            name: name.to_string(),
            ipa,
            phones_mask,
            id,
        }
    }
}

/// Clean up pronunciations
pub fn clean_pronunciation(mut i: &str) -> String {
    // Trim whitespace characters
    i = i.trim_matches(|c: char| c == '/' || c.is_ascii_whitespace());

    // Remove Nidoran gender (appears in brackets)
    if let Some((j, _)) = i.split_once('(') {
        i = j.trim();
    }

    let mut i = i.to_string();

    // Use single codepoints for digraphs
    i = i.replace("\u{329}h", "ḥ"); // not correct, but is a single codepoint
    i = i.replace("dʒ", "ʤ");
    i = i.replace("tʃ", "ʧ");
    i = i.replace("oʊ", "ö");
    i = i.replace("aɪ", "ï");
    i = i.replace("eɪ", "ä");
    i = i.replace("ɔɪ", "õ");
    i = i.replace("aʊ", "Ø");

    i = i.replace("ɑɪ", "ï"); //Some names (arcanine, omanyte) used the wrong diphthong
    i = i.replace("r", "ɹ"); //English has no r, only ɹ
    i = i.replace("g", "ɡ"); //Use ɡ (the actual phoneme), not g
    i = i.replace("a", "ɑ"); //Use ɑ (the actual phoneme), not a

    i = i.replace("ɾ", "d"); //GA treats ɾ as d
    i = i.replace("ʌ", "ə"); //GA treats ʌ as ə
    i = i.replace("ɜ", "ə"); //GA treats ɜ as ə
    i = i.replace("ɝ", "ɚ"); //GA treats ɜr as ər
    i = i.replace("ɔ", "ɑ"); //GA treats ɔ as ɑ

    // Remove things that aren't phones
    let non_phoeneme = " ːˈˌ";
    i = i.replace(|p| non_phoeneme.contains(p), "");

    i
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cleanup() {
        assert_eq!("ˈniːdöɹæn", clean_pronunciation("/ˈniːdoʊɹæn (ˈfiːmeɪl)/"));
        assert_eq!("ˈʤɪɡliːpəf", clean_pronunciation("/ˈdʒɪɡliːpʌf/"));
    }
}
