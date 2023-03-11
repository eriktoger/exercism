#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }
    for num in number {
        if *num >= from_base {
            return Err(Error::InvalidDigit(*num));
        }
    }
    let mut sum = number.iter().fold(0, |acc, num| acc * from_base + num);

    let mut answer = vec![];
    while sum > 0 {
        answer.push(sum % to_base);
        sum /= to_base;
    }

    if answer.is_empty() {
        return Ok(vec![0]);
    }
    answer.reverse();
    Ok(answer)
}
