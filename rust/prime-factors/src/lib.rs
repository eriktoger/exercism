use std::cmp::min;

const SIEVE_LIMIT: u64 = 10000;

fn sieve(n: u64) -> Vec<u64> {
    let upper_limit = min(n + 2, SIEVE_LIMIT);
    let mut primes: Vec<u64> = (2_u64..upper_limit).collect();
    let mut idx = 0;
    let mut prime_counter = 0;
    loop {
        let current_prime = primes[prime_counter];
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
        }
    }
    primes
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let primes = sieve(n);
    let mut prime_factors: Vec<u64> = vec![];

    for prime in primes.iter() {
        while n % prime == 0 {
            prime_factors.push(*prime);
            n /= prime;
        }

        if n == 1 {
            return prime_factors;
        }
    }

    prime_factors.push(n);
    prime_factors
}
