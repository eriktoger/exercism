pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || digits.len() < len {
        return vec![];
    }
    let mut result = Vec::new();
    let end_position = digits.len() - len;
    for i in 0..=end_position {
        let sub_string = &digits[i..i + len];
        result.push(sub_string.to_string());
    }

    result
}
