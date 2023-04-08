use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<ARR, KEY>(arr: ARR, key: KEY) -> Option<usize>
where
    KEY: Ord,
    ARR: AsRef<[KEY]>,
{
    let array = arr.as_ref();
    if array.is_empty() {
        return None;
    }

    let mut lower_limit = 0;
    let mut upper_limit = array.len() - 1;
    let mut current_index = (lower_limit + upper_limit) / 2;

    loop {
        let current_key = &array[current_index];
        let old_index = current_index;

        match key.cmp(current_key) {
            Equal => {
                return Some(current_index);
            }
            Less => {
                upper_limit = current_index;
            }
            Greater => {
                lower_limit = current_index + 1;
            }
        };

        current_index = (lower_limit + upper_limit) / 2;

        if old_index == current_index {
            return None;
        }
    }
}
