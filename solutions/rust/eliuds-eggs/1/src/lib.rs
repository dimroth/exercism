pub fn egg_count(display_value: u32) -> usize {
    let mut stack = Vec::<u32>::new();
    let mut number = display_value;

    while number > 0 {
        let remainder = number % 2;
        stack.push(remainder);
        number /= 2;
    }

    stack.iter().fold(0, |acc, x| acc + x) as usize
}
