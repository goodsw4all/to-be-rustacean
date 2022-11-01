/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // too slow
    let mut palindroms = Vec::new();

    for i in min..=max {
        for j in i..=max {
            if is_palindrome(i * j) {
                palindroms.push(i * j);
            }
        }
    }

    if palindroms.len() < 2 {
        return None;
    }
    palindroms.sort();

    let max = Palindrome::new(palindroms[palindroms.len() - 1]);
    let min = Palindrome::new(palindroms[0]);

    Some((min.unwrap(), max.unwrap()))
}

fn is_palindrome(num: u64) -> bool {
    let mut n = num;
    let mut sum = 0_u64;

    while n > 0 {
        sum = sum * 10 + n % 10;
        n /= 10;
    }
    sum == num
}
