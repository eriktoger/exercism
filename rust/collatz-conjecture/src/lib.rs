fn inner_collatz(n: u64, steps: u64) -> Option<u64> {
    if n == 1 {
        return Some(steps);
    }
    let new_n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    inner_collatz(new_n, steps + 1)
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    inner_collatz(n, 0)
}
