pub fn factors_of(n: u32) -> Vec<u32> {
    (1..=n).into_iter().filter(|&x| n % x == 0).collect()
}