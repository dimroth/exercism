pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = Vec::<u32>::new();
    for i in 1..limit {
        for factor in factors.iter().filter(|f| **f != 0) {
            if i % factor == 0 {
                set.push(i);
            }
        }
    }
    set.dedup();
    set.iter().sum()
}
