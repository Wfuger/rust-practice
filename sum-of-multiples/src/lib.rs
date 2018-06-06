pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| is_multiple(i, factors))
        .fold(0, |sum, x| sum + x)
}

fn is_multiple(n: &u32, factors: &[u32]) -> bool {
    factors
        .iter()
        .filter(|&f| n % f == 0)
        .collect::<Vec<&u32>>()
        .len() > 0
}
