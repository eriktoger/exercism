const ALL_TYPES: [char; 6] = ['(', ')', '[', ']', '{', '}'];
const OPEN_TYPES: [char; 3] = ['(', '[', '{'];
const CLOSE_TYPES: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let first_bracket = match string
        .chars()
        .enumerate()
        .find(|(_, c)| ALL_TYPES.iter().find(|t| *t == c).is_some())
    {
        Some(bracket) => bracket,
        None => return true,
    };

    if CLOSE_TYPES
        .iter()
        .find(|c| **c == first_bracket.1)
        .is_some()
    {
        return false;
    }

    let bracket_idx = OPEN_TYPES
        .iter()
        .enumerate()
        .find(|c| *c.1 == first_bracket.1)
        .unwrap()
        .0;

    let mut open_counter = 1;
    let start = first_bracket.0 + 1;
    let end = string.len();
    for idx in start..end {
        let chr = string.chars().nth(idx).unwrap();
        if chr == OPEN_TYPES[bracket_idx] {
            open_counter += 1;
        } else if chr == CLOSE_TYPES[bracket_idx] {
            open_counter += -1;

            if open_counter == 0 {
                return brackets_are_balanced(&string[start..idx])
                    && brackets_are_balanced(&string[(idx + 1)..end]);
            }
        }
    }

    open_counter == 0
}
