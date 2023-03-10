const SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let sound = SOUNDS
        .iter()
        .fold(String::new(), |mut acc, (prime, sound)| {
            if n % prime == 0 {
                acc += sound
            }
            acc
        });
    if sound == String::new() {
        return n.to_string();
    }
    sound
}
