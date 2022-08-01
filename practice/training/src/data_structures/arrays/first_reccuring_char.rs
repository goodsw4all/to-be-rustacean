use std::collections::{hash_map::Entry, HashMap};

fn first_recurring_char_entry(word: &str) -> Option<char> {
    let mut hashmap = HashMap::new();
    for c in word.chars() {
        if let Entry::Occupied(_) = hashmap.entry(c) {
            return Some(c);
        } else {
            hashmap.insert(c, 1);
        }
    }

    return None;
}

fn first_recurring_char_insert(word: &str) -> Option<char> {
    let mut hashmap = HashMap::new();
    for c in word.chars() {
        if let Some(_) = hashmap.insert(c, 1) {
            return Some(c);
        }
    }

    return None;
}

fn first_recurring_char(word: &str) -> Option<char> {
    // first_recurring_char_entry(word)
    first_recurring_char_insert(word)
}

#[test]
fn test_first_recurring_char() {
    assert_eq!(first_recurring_char("abcdk"), None);
    assert_eq!(first_recurring_char("abcdc"), Some('c'));
    assert_eq!(first_recurring_char("akkkkkkkkk"), Some('k'));
}
