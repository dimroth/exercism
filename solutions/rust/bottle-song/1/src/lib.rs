const NUMBERS_STR: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut lyrics = String::new();
    for i in (start_bottles - take_down..start_bottles).rev() {
        let mut verse = String::new();
        let num_str = NUMBERS_STR[(i + 1) as usize];
        let num_str_next = NUMBERS_STR[i as usize];
        verse.push_str(&format!("{} green bottle{} hanging on the wall,\n{} green bottle{} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green bottle{} hanging on the wall.", num_str, if num_str == "One" { "" } else { "s" }, num_str, if num_str == "One" { "" } else { "s" }, num_str_next.to_lowercase(), if num_str_next == "One" { "" } else { "s" }));
        lyrics.push_str(&verse);
        if i != start_bottles - take_down {
            lyrics.push_str("\n\n");
        }
    }
    lyrics
}
