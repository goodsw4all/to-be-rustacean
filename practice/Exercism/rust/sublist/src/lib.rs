#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

use std::fmt::Debug;

pub fn sublist<T: PartialEq + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_list_len = _first_list.len();
    let second_list_len = _second_list.len();

    // Check Equal
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    // println!("{:?}", _first_list);

    // Check Sublist
    if first_list_len < second_list_len {
        for idx in 0..=(second_list_len - first_list_len) {
            if &_second_list[idx..(idx + first_list_len)] == _first_list {
                return Comparison::Sublist;
            }
        }
    }

    // Check Superlist
    if first_list_len > second_list_len {
        for idx in 0..=(first_list_len - second_list_len) {
            if &_first_list[idx..(idx + second_list_len)] == _second_list {
                return Comparison::Superlist;
            }
        }
    }

    // rest of cases
    return Comparison::Unequal;
}
