pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut counts = vec![0; 6];

    for &bird_id in arr.iter() {
        counts[bird_id as usize] += 1;
    }

    let mut max_count = 0;
    let mut result_id = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result_id = i as i32;
        }
    }

    result_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        assert_eq!(migratory_birds(vec![1, 4, 4, 4, 5, 3]), 4);
        assert_eq!(migratory_birds(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }
}