pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    for x in max_a..=min_b {
        let is_a_factors = a.iter().all(|&val| x % val == 0);
        let is_x_factor_of_b = b.iter().all(|&val| val % x == 0);

        if is_a_factors && is_x_factor_of_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(vec![2, 4], vec![16, 32, 96]), 3);
        assert_eq!(get_total_x(vec![2, 6], vec![24, 36]), 2);
    }
}