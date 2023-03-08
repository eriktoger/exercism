pub fn calc_square(s: u32) -> u128 {
    2_u128.pow(s - 1)
}

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    calc_square(s) as u64
}

pub fn total() -> u128 {
    calc_square(65) - 1
}
