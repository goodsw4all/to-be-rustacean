/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn
        .chars()
        .filter(|c| c != &'-')
        .enumerate()
        .filter(|(idx, c)| c.is_numeric() || (*c == 'X' && *idx == 9))
        .count()
        != 10
    {
        return false;
    }

    isbn.chars()
        .filter(|c| c.is_numeric() || c == &'X')
        .zip((1..=10).rev())
        .fold(0, |acc, (c, n)| {
            if c == 'X' {
                10 * n + acc
            } else {
                c.to_digit(10).unwrap() * n + acc
            }
        })
        % 11
        == 0
}
