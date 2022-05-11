const TIME_IN_OVEN: i32 = 40;
pub fn expected_minutes_in_oven() -> i32 {
    TIME_IN_OVEN
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    TIME_IN_OVEN - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)
}
