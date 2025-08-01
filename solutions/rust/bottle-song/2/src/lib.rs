const NUMBERS_STR: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

fn number_to_word(num: u32) -> &'static str {
    NUMBERS_STR[num as usize]
}

fn plural_suffix(num: u32) -> &'static str {
    if num == 1 { "" } else { "s" }
}

fn generate_verse(bottles: u32) -> String {
    let current_word = number_to_word(bottles);
    let remaining_bottles = bottles.saturating_sub(1);
    let remaining_word = number_to_word(remaining_bottles);
    
    format!(
        "{} green bottle{} hanging on the wall,\n\
         {} green bottle{} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {} green bottle{} hanging on the wall.",
        current_word,
        plural_suffix(bottles),
        current_word,
        plural_suffix(bottles),
        remaining_word.to_lowercase(),
        plural_suffix(remaining_bottles)
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();
    
    for bottles in (start_bottles - take_down + 1..=start_bottles).rev() {
        verses.push(generate_verse(bottles));
    }
    
    verses.join("\n\n")
}
