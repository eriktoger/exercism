// number to word
fn number_to_word(n: u32) -> String {
    let word = match n {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "",
    };
    word.to_string()
}

fn uppercase_first_letter(str: String) -> String {
    let mut v: Vec<char> = str.chars().collect();
    v[0] = v[0].to_uppercase().next().unwrap();
    v.into_iter().collect()
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    for i in 0..take_down {
        let bottle_number = start_bottles - i;
        let bottles = number_to_word(bottle_number);
        let uppercase_bottles = uppercase_first_letter(bottles);
        let bottle_plural_s = if bottle_number == 1 { "" } else { "s" };

        let next_bottle_number = start_bottles - i - 1;
        let next_bottles = number_to_word(next_bottle_number);
        let next_bottle_plural_s = if next_bottle_number == 1 { "" } else { "s" };

        let new_line = if i == take_down - 1 { "" } else { "\n" };

        let verse = format!(
            "{bottles} green bottle{plural} hanging on the wall,\n\
             {bottles} green bottle{plural} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {next_bottles} green bottle{next_plural} hanging on the wall.\n{new_line}",
            bottles = uppercase_bottles,
            plural = bottle_plural_s,
            next_bottles = next_bottles,
            next_plural = next_bottle_plural_s,
            new_line = new_line
        );

        song.push_str(&verse);
    }
    song.trim_end_matches('\n').to_string()
}
