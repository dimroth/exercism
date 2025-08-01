pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::<u64>::new();
    let mut divisor = 2;
    let mut number = n;
    while number > 1 {
        while number % divisor == 0 {
            prime_factors.push(divisor);
            number /= divisor;
        }
        divisor += 1;
    }
    prime_factors
}
