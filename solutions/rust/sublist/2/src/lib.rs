#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        (first, second) if first == second => Comparison::Equal,
        (first, _) if first.is_empty() => Comparison::Sublist,
        (_, second) if second.is_empty() => Comparison::Superlist,
        (first, second) if is_sublist(first, second) => Comparison::Sublist,
        (first, second) if is_sublist(second, first) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist(smaller: &[i32], larger: &[i32]) -> bool {
    if smaller.len() > larger.len() {
        false;
    }
    larger
        .windows(smaller.len())
        .any(|window| window == smaller)
}
