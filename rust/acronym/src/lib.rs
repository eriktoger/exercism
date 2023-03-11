pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'])
        .filter(|s| !s.is_empty())
        .fold("".to_string(), |acc, x| {
            if !x.chars().any(char::is_lowercase) {
                return acc + &x.chars().next().unwrap().to_string();
            }

            acc + &x
                .chars()
                .enumerate()
                .filter(|(idx, c)| *idx == 0 || c.is_uppercase())
                .map(|(_, c)| c)
                .collect::<String>()
        })
        .to_uppercase()
}
