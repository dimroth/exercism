use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, result) = parse_equation(input)?;
    let leading_chars = get_leading_chars(&addends, result);
    let coeffs = calculate_coeffs(&addends, result);
    
    let mut letters: Vec<char> = coeffs.keys().cloned().collect();
    letters.sort_by_key(|&c| -(coeffs[&c].abs() as i128));

    let mut assignment = HashMap::new();
    let mut used = [false; 10];

    if backtrack(0, 0, &letters, &coeffs, &leading_chars, &mut assignment, &mut used) {
        Some(assignment)
    } else {
        None
    }
}

fn parse_equation(input: &str) -> Option<(Vec<&str>, &str)> {
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }

    let addends: Vec<&str> = parts[0].split(" + ").map(|s| s.trim()).collect();
    let result = parts[1].trim();

    Some((addends, result))
}

fn get_leading_chars(addends: &[&str], result: &str) -> HashSet<char> {
    let mut leading = HashSet::new();
    for word in addends {
        if let Some(first) = word.chars().next() {
            leading.insert(first);
        }
    }
    if let Some(first) = result.chars().next() {
        leading.insert(first);
    }
    leading
}

fn calculate_coeffs(addends: &[&str], result: &str) -> HashMap<char, i64> {
    let mut coeffs = HashMap::new();
    for word in addends {
        for (i, c) in word.chars().rev().enumerate() {
            *coeffs.entry(c).or_insert(0) += 10i64.pow(i as u32);
        }
    }
    for (i, c) in result.chars().rev().enumerate() {
        *coeffs.entry(c).or_insert(0) -= 10i64.pow(i as u32);
    }
    coeffs
}

fn backtrack(
    k: usize,
    current_sum: i64,
    letters: &[char],
    coeffs: &HashMap<char, i64>,
    leading_chars: &HashSet<char>,
    assignment: &mut HashMap<char, u8>,
    used: &mut [bool; 10],
) -> bool {
    if k == letters.len() {
        return current_sum == 0;
    }

    let letter = letters[k];
    let coeff = coeffs[&letter];

    for digit in 0..10 {
        if used[digit] {
            continue;
        }

        if digit == 0 && leading_chars.contains(&letter) {
            continue;
        }

        assignment.insert(letter, digit as u8);
        used[digit] = true;

        if backtrack(k + 1, current_sum + coeff * (digit as i64), letters, coeffs, leading_chars, assignment, used) {
            return true;
        }

        used[digit] = false;
        assignment.remove(&letter);
    }

    false
}