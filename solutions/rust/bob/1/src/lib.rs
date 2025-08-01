pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    
    // Check for silence first
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }
    
    // Check if it's a question (ends with ?)
    let is_question = trimmed.ends_with('?');
    
    // Check if it's shouting (has letters and all letters are uppercase)
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_shouting = has_letters && trimmed.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    
    match (is_question, is_shouting) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
