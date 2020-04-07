pub fn convert_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first: char = chars.next().unwrap();
    let rest: &str = chars.as_str();
    if is_vowel(first) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", rest, first)
    }
}

fn is_vowel(c: char) -> bool {
    String::from("aeiou").contains(c)
}
