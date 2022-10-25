pub fn collatz(n: u64) -> Option<u64> {
    let mut collatz = n;
    let mut steps = 0;

    loop {
        collatz = match collatz {
            0 => return None,
            1 => break,
            c if c % 2 == 0 => c / 2,
            _ => collatz.checked_mul(3)?.checked_add(1)?,
        };
        steps += 1;
    }

    Some(steps)
}
