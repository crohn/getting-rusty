// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and ay is added, so first become irst-fay. Words that
// start with a vowel have a hay added to the end instead (apple becomes
// apple-hay). Keep in mind the details about UTF-8 encoding!

use getting_rusty::string::to_pig_latin;

fn main() {
    let word = to_pig_latin("first");
    assert_eq!(word, "irst-fay");

    let word = to_pig_latin("apple");
    assert_eq!(word, "apple-hay");

    let word = to_pig_latin("f");
    assert_eq!(word, "fay");

    let word = to_pig_latin("a");
    assert_eq!(word, "a-hay");

    let word = to_pig_latin("あ");
    assert_eq!(word, "あay");

    let word = to_pig_latin("á");
    assert_eq!(word, "á-hay");
}
