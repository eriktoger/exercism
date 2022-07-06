/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    let only_digits = trimmed.chars().all(|c| char::is_digit(c, 10));
    if trimmed == "0" || !only_digits {
        return false;
    }
    let mut sum = 0;
    for (i, c) in trimmed.chars().rev().enumerate() {
        sum += double_every_second_char(c, i);
    }
    return sum % 10 == 0;
}

fn double_every_second_char(c: char, i: usize) -> u32 {
    let mut integer = c.to_digit(10).unwrap();
    if i % 2 == 1 {
        integer *= 2;
        if integer > 9 {
            integer -= 9;
        }
    }
    return integer;
}
