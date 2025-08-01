pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|word| {
            let mut result = Vec::new();
            let chars: Vec<char> = word.chars().collect();
            
            if chars.is_empty() {
                return result;
            }
            
            if chars[0].is_alphabetic() {
                result.push(chars[0].to_ascii_uppercase());
            }
            
            if !word.chars().all(|c| c.is_ascii_uppercase() || !c.is_alphabetic()) && chars.len() > 1 {
                for &c in &chars[1..] {
                    if c.is_ascii_uppercase() && c.is_alphabetic() {
                        result.push(c);
                    }
                }
            }
            
            result
        })
        .collect()
}
