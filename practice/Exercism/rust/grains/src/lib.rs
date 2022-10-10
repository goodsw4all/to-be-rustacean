pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => (2..=s).fold(1, |acc, _| acc * 2),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
