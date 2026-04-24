pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        // Приклад: s=7, t=11, a=5, b=15, яблука=[-2, 2, 1], апельсини=[5, -6]
        let (apples_res, oranges_res) = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
        assert_eq!(apples_res, 1);
        assert_eq!(oranges_res, 1);
    }
}