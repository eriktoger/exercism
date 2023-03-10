pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let non_zero_factors: Vec<&u32> = factors.iter().filter(|x| **x != 0).collect();
    (1..limit).into_iter().fold(0, |acc, num| {
        acc + match non_zero_factors.iter().any(|factor| num % **factor == 0) {
            true => num,
            _ => 0,
        }
    })
}
