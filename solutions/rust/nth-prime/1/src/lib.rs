pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    
    let mut primes = vec![2];
    let mut candidate = 3;
    
    while primes.len() <= n as usize {
        let mut is_prime = true;
        let sqrt_candidate = (candidate as f64).sqrt() as u32;
        
        // Check if candidate is prime by testing divisibility up to sqrt(candidate)
        for &prime in &primes {
            if prime > sqrt_candidate {
                break;
            }
            if candidate % prime == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            primes.push(candidate);
        }
        
        candidate += 2; // Only test odd numbers (except 2)
    }
    
    primes[n as usize]
}
