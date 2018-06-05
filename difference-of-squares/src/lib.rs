pub fn square_of_sum(n: usize) -> usize {
    let sum = (1..=n).fold(0, |accum, x| accum + x);
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..=n).map(|x| x * x).fold(0, |sum, x| sum + x)
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
