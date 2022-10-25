fn is_prime_number(n: u32) -> bool {
    !(2..=(n as f32).sqrt() as u32).any(|x| n % x == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..u32::MAX)
        .filter(|&x| is_prime_number(x))
        .nth(n as usize)
        .expect("error")
}
