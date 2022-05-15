use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();
    for word in magazine {
        words
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for word in note {
        let count = words.get_mut(word);
        match count {
            Some(val) => match val {
                val if *val > 0 => *val -= 1,
                _ => return false,
            },
            None => return false,
        }
    }
    return true;
}
