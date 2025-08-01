const NAMES: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

fn get_plant(letter: char) -> &'static str {
    match letter {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Invalid plant letter: {}", letter),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result: Vec<&'static str> = Vec::new();
    let student_index = NAMES.iter().position(|name| name == &student).unwrap();

    let first_row = diagram.split('\n').next().unwrap();
    let second_row = diagram.split('\n').nth(1).unwrap();

    first_row.chars().skip(student_index * 2).take(2).map(|c| get_plant(c)).for_each(|p| result.push(p));
    second_row.chars().skip(student_index * 2).take(2).map(|c| get_plant(c)).for_each(|p| result.push(p));

    result
}
