pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string().chars().collect::<Vec<char>>();
    let num_len = num_str.len();
    let sum: u32 = num_str.iter().map(|c| c.to_digit(10).unwrap()).map(|d| d.pow(num_len as u32)).sum();
    sum == num
}
