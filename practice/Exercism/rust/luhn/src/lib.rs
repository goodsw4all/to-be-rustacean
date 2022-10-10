/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let numbers = code.split_whitespace().collect::<String>();

    let mut sum = 0;

    if numbers.len() <= 1 || numbers.chars().any(|x| !x.is_numeric()) {
        return false;
    }

    for (idx, digit) in numbers.chars().rev().enumerate() {
        if let Some(n) = digit.to_digit(10) {
            if idx % 2 == 1 {
                sum += if 2 * n > 9 { 2 * n - 9 } else { 2 * n };
            } else {
                sum += n;
            }
        } else {
            return false;
        }
    }

    sum % 10 == 0
}
