pub fn sum_of_multiples_for_loop(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = std::collections::HashSet::new();

    for num in 1..limit {
        for devisor in factors.iter() {
            if num >= *devisor && devisor > &0 {
                if (num % devisor) == 0 {
                    multiples.insert(num);
                    break;
                }
            }
        }
    }

    multiples.iter().sum()
}

pub fn sum_of_multiples_filter(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| {
            factors
                .iter()
                .any(|divisor| divisor != &0 && num % divisor == 0)
        })
        .sum()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    sum_of_multiples_filter(limit, factors)
}
