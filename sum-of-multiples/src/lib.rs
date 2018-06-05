pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| is_multiple(i, factors))
        // .collect::<Vec<u32>>()
        .fold(0, |sum, x| sum + x)
}

fn is_multiple(n: &u32, factors: &[u32]) -> bool {
    let mut multiple = false;
    for f in factors {
        if n % f == 0 {
            multiple = true;
            break;
        }
    }
    return multiple;
}
