use std::collections::HashMap;

fn count_occurrences_forloop(word: &str) -> HashMap<char, i32> {
    let mut hashmap = HashMap::new();

    for c in word.chars() {
        let count = hashmap.entry(c).or_insert(0);
        *count += 1;
    }

    hashmap
}

fn count_occurrences_functional(word: &str) -> HashMap<char, i32> {
    let mut hashmap = HashMap::new();

    word.chars()
        .for_each(|c| *hashmap.entry(c).or_insert(0) += 1);

    hashmap
}

fn count_occurrences(word: &str) -> HashMap<char, i32> {
    // count_occurrences_forloop(word)
    count_occurrences_functional(word)
}

#[test]
fn test_count_occurrences() {
    assert_eq!(count_occurrences(""), HashMap::from([]));

    assert_eq!(
        count_occurrences("abc"),
        HashMap::from([('a', 1), ('b', 1), ('c', 1),])
    );
}
