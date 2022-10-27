pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace('-', " ")
        .replace('_', " ")
        .split(|c: char| c.is_whitespace())
        .flat_map(|s| {
            s.chars().take(1).chain(
                s.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
