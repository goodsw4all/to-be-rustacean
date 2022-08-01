// Given 2 arrays, create a function that let's a user know (true/false) whether these two arrays contain any common items
//For Example:
//const array1 = ['a', 'b', 'c', 'x'];//const array2 = ['z', 'y', 'i'];
//should return false.
//-----------
//const array1 = ['a', 'b', 'c', 'x'];//const array2 = ['z', 'y', 'x'];
//should return true.

// 2 parameters - arrays - no size limit
// return true or false

use std::collections::HashSet;

// O(N^2) O(1)
fn contains_common_item_bruteforce(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    for arr1_item in arr1.iter() {
        for arr2_item in arr2.iter() {
            if *arr1_item == *arr2_item {
                return true;
            }
        }
    }
    false
}

// O(2N) O(N)
fn contains_common_item_hashset(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    let mut contained_items_map: HashSet<char> = HashSet::new();

    // iter() iterates over the items by reference
    // iter_mut() iterates over the items, giving a mutable reference to each item
    // into_iter() iterates over the items, moving them into the new scope
    for item in arr1.iter() {
        contained_items_map.insert(*item);
    }

    for item in arr2.iter() {
        if contained_items_map.contains(item) {
            return true;
        }
    }
    false
}

fn contains_common_item_rusty(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    arr1.iter().any(|x| arr2.contains(x)) // contains O(n)
}

fn contains_common_item(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    // contains_common_item_bruteforce(arr1, arr2)
    contains_common_item_hashset(arr1, arr2)
    //contains_common_item_rusty(arr1, arr2)
}

#[test]
// #[ignore]
fn test_time_measure() {
    // Reason: small input data, debug mode benchmark, not proper benchmark method 
    // Time elapsed in function bruteforce is: 2.387µs
    // Time elapsed in function hashset is: 24.413µs // Why it takes relatively long ?
    // Time elapsed in function rusty is: 1.13µs
    // test arrays::contains_common_item::test_time_measure ... ok
    let alphabet = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z',
    ];

    let arr2 = vec!['z', 'y', 'i'];
    let start = std::time::Instant::now();
    contains_common_item_bruteforce(alphabet.clone(), arr2);
    let duration: std::time::Duration = start.elapsed();
    eprintln!("Time elapsed in function bruteforce is: {:?}", duration);

    let arr2 = vec!['z', 'y', 'i'];
    let start = std::time::Instant::now();
    contains_common_item_hashset(alphabet.clone(), arr2);
    let duration: std::time::Duration = start.elapsed();
    eprintln!("Time elapsed in function hashset is: {:?}", duration);

    let arr2 = vec!['z', 'y', 'i'];
    let start = std::time::Instant::now();
    contains_common_item_rusty(alphabet.clone(), arr2);
    let duration: std::time::Duration = start.elapsed();
    eprintln!("Time elapsed in function rusty is: {:?}", duration);
}

#[test]
fn test_contains_common_item() {
    let array1 = vec!['a', 'b', 'c', 'x'];
    let array2 = vec!['z', 'y', 'i'];
    assert_eq!(contains_common_item(array1, array2), false);

    let array1 = vec!['a', 'b', 'c', 'x'];
    let array2 = vec!['z', 'y', 'x'];
    assert_eq!(contains_common_item(array1, array2), true);

    let array1 = vec![];
    let array2 = vec!['z', 'y', 'x'];
    assert_eq!(contains_common_item(array1, array2), false);
}
