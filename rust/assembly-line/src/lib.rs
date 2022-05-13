// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
const HOURLY_RATE: f64 = 221.0;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = speed as f64 * HOURLY_RATE;
    match speed {
        0..=4 => rate,
        5..=8 => rate * 0.90,
        9.. => rate * 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
