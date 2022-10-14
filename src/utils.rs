use crate::extras;

pub fn squarest_rect_with_even_area(n: u32) -> [u32; 2] {
    let area = 2 * n;
    let factors = extras::factors_of(area);

    if factors.is_empty() {
        return [0; 2];
    }
    let mid = factors.len() / 2;
    if factors.len() % 2 == 0 {
        return factors[mid - 1..=mid].try_into().unwrap();
    }
    [factors[mid]; 2]
}
