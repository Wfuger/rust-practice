pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let base: u64 = 2;
    base.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |sum, x| sum + square(x))
}
