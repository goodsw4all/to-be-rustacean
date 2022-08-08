fn sum(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    return sum(n - 1) + n;
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_sum() {
        let expected = 1;
        assert_eq!(sum(1), expected);

        let expected = 55;
        assert_eq!(sum(10), expected);

        let expected = 5050;
        assert_eq!(sum(100), expected);
    }
}
