use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagram_set = HashSet::new();
    let word_lower = word.to_lowercase();
    
    for candidate in possible_anagrams {
        if candidate.to_lowercase() == word_lower {
            continue;
        }
        
        if candidate.len() == word.len() {
            // Convert to lowercase for comparison, then sort characters
            let mut word_chars: Vec<char> = word_lower.chars().collect();
            let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();
            
            word_chars.sort_unstable();
            candidate_chars.sort_unstable();
            
            // If sorted characters match, it's an anagram
            if word_chars == candidate_chars {
                anagram_set.insert(*candidate);
            }
        }
    }
    
    anagram_set
}
