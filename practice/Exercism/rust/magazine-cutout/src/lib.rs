// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::{collections::HashMap, hash::Hash};
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dictionary: HashMap<&str, u32> = HashMap::new();

    for word in magazine.iter() {
        let dic_entry = dictionary
            .entry(word)
            .or_insert(0); // entry없으면 key, value 추가
        *dic_entry += 1; // count 증가
    }

    for word in note.iter() {
        if dictionary.contains_key(word) == false {
            return false;
        }

        let dic_entry = dictionary
            .entry(word)
            .or_insert(0);

        if *dic_entry == 0 {
            return false;
        }
        *dic_entry -= 1
    }
    return true;
}
