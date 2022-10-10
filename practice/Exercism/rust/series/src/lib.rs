pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::from(""); digits.len() + 1];
    }
    let char_arrays = digits.chars().collect::<Vec<char>>();
    char_arrays
        .windows(len)
        .map(|x| x.iter().collect::<String>())
        .collect()
}
