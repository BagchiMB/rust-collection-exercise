pub fn calculate_pig_latin_equivalent(word: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_char = word.chars().next().unwrap();

    if vowels.contains(&first_char) {
        return format!("{word}-hay");
    } else {
        return format!("{}-{first_char}ay", &word[1..]);
    }
}
