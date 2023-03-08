pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.clone().to_string().len() as u32;

    let sum = num
        .to_string()
        .chars()
        .into_iter()
        .map(|x| x.to_digit(10).unwrap().pow(length) as u64)
        .sum::<u64>();
    sum == (u64::from(num))
}
