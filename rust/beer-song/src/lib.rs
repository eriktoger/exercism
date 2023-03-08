const PART_ONE: &str = " bottles of beer on the wall, ";
const PART_TWO: &str = " bottles of beer.\nTake one down and pass it around, ";
const PART_THREE: &str = " bottles of beer on the wall.\n";

const VERSE_ZERO: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const VERSE_ONE:&str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const VERSE_TWO:&str ="2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

fn get_generic_verse(n: u32) -> String {
    n.to_string() + PART_ONE + &n.to_string() + PART_TWO + &(n - 1).to_string() + PART_THREE
}

pub fn verse(n: u32) -> String {
    match n {
        0 => VERSE_ZERO.to_string(),
        1 => VERSE_ONE.to_string(),
        2 => VERSE_TWO.to_string(),
        _ => get_generic_verse(n),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = "".to_string();
    for n in (end..(start + 1)).rev() {
        song = format!("{}{}", song, verse(n));
        if n != end {
            song = format!("{}{}", song, "\n");
        }
    }
    song
}
