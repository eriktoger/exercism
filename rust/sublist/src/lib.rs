#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Would be a lot easier if I knew windows was a thing...
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list {
            return Comparison::Equal;
        }
        return Comparison::Unequal;
    }
    let (short_list, long_list, list_type) = get_lists(_first_list, _second_list);
    if short_list.is_empty() {
        return list_type;
    }

    let mut index = 0;
    loop {
        let end = index + short_list.len();
        if end > long_list.len() {
            return Comparison::Unequal;
        }
        let slice = &long_list[index..end];
        if slice == short_list {
            return list_type;
        }
        index += 1;
    }
}

fn get_lists<'a, T: PartialEq>(first: &'a [T], second: &'a [T]) -> (&'a [T], &'a [T], Comparison) {
    if first.len() > second.len() {
        return (second, first, Comparison::Superlist);
    }
    (first, second, Comparison::Sublist)
}
