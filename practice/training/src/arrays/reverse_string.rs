fn reverse_string_array(s: String) -> String {
    let mut reversed_string = String::new();
    // Indexing on String is not allowed
    let s_chars = s.chars().collect::<Vec<char>>();
    for idx in (0..s.len()).rev() {
        reversed_string.push(s_chars[idx])
    }

    reversed_string
}

fn reverse_string_rusty(s: String) -> String {
    s.chars().rev().collect::<String>()
}

fn reverse_string(s: String) -> String {
    reverse_string_array(s)
    // reverse_string_rusty(s)
}

#[test]
fn test_reverse_string() {
    let s = String::from("hi");
    assert_eq!(reverse_string(s), "ih");

    let s = String::from("hello world");
    assert_eq!(reverse_string(s), "dlrow olleh");

    let s = String::from("adsfadfa");
    assert_ne!(reverse_string(s), "adfaddf");
}
