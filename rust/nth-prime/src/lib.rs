const LIMIT: u32 = 110_000;
const SIEVE_LIMIT: u32 = 350; // sqrt(LIMIT)

fn sieve() -> Vec<u32> {
    let mut primes: Vec<u32> = (2_u32..SIEVE_LIMIT).collect();
    let mut idx = 0;
    let mut current_prime = primes[idx];
    let mut prime_counter = 0;
    loop {
        let is_not_prime = primes[idx] % current_prime == 0 && primes[idx] != current_prime;
        if is_not_prime {
            primes.remove(idx);
        } else {
            idx += 1;
        }

        if idx == primes.len() {
            prime_counter += 1;
            if prime_counter == primes.len() {
                break;
            }
            idx = prime_counter;
            current_prime = primes[prime_counter];
        }
    }
    primes
}

pub fn nth(n: usize) -> u32 {
    let primes = sieve();
    if n < primes.len() {
        return primes[n];
    }

    let numbers: Vec<u32> = (SIEVE_LIMIT..LIMIT).collect();
    let mut counter = primes.len();
    for num in numbers.iter() {
        let mut is_prime = true;
        for prime in primes.iter() {
            if num % prime == 0 && num != prime {
                is_prime = false;
                break;
            }
            if prime * prime >= *num {
                break;
            }
        }
        if is_prime {
            counter += 1;
        }
        if counter > n {
            return *num;
        }
    }
    panic!("{}-th prime not found. Try increasing the limits", n);
}
