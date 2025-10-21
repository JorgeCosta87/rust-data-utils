const CONSONANT_SUFFIX: &str = "ay";
const VOWEL_SUFFIX: &str = concat!("h", "ay");
const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

pub fn convert_to_pig_latin(str: &str) -> String{
    if str.starts_with(VOWELS) {
        format!("{}-{}", str, VOWEL_SUFFIX)
    } else {
        let first_letter = &str[0..1];
        format!( "{}-{}{}" , &str[1..], &first_letter, CONSONANT_SUFFIX)
    }
}

pub fn safe_convert_to_pig_latin(str: &str) -> String{
    let mut chs = str.chars();
    let first_char = chs.next().unwrap();
    
    if VOWELS.contains(&first_char.to_ascii_lowercase()) {
        format!("{}-{}", str, VOWEL_SUFFIX)
    } else {
        let rest: String = chs.collect();
        format!("{}-{}{}", rest, first_char, CONSONANT_SUFFIX)
    }
}