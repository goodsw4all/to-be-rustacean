use std::os::unix::prelude::IntoRawFd;

pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = vec![];
    let mut divider = 2;

    while n > 1 {
        while n % divider == 0 {
            factors.push(divider);
            n /= divider;
        }
        divider += 1;
    }
    
    factors
}
