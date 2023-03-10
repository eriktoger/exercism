pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let is_question = trimmed.ends_with("?");
    let all_caps = trimmed.chars().all(|c| !c.is_ascii_lowercase())
        && trimmed.chars().any(|c| c.is_ascii_uppercase());
    let silence = trimmed.trim().is_empty();

    if is_question && all_caps {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if silence {
        "Fine. Be that way!"
    } else if all_caps {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
