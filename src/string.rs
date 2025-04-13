/// Returns the pig latin version of the provided word.
///
/// By spec, pig latin words work like this:
///   - if the original word starts with consonant
///     -> the resulting word has the consonant moved to the end, after a dash,
///        and 'ay' is added. Example: first -> irst-fay
///   - if the original word starts with vowel
///     -> the resulting word is kept as is, and 'hay' is added after a dash.
///        Example: apple -> apple-hay
///
/// A single consonant is handled by just adding 'ay', so the resulting word
/// does not start with a dash.
///
/// The function is UTF-8 aware, but treats non-ASCII vowels as consonants.
pub fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        // NOTE I peeked on Chapter 19.3 and found Match Guards, which allowed
        // to rewrite using a single match statement
        Some(first) if is_vowel(first) => format!("{}-hay", word),
        Some(first) => {
            let rest: String = chars.collect();

            if rest.is_empty() {
                format!("{}ay", first)
            } else {
                format!("{}-{}ay", rest, first)
            }
        }
        None => String::new(),
    }
}

/// Returns true if the provided char is a latin vowel.
pub fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
