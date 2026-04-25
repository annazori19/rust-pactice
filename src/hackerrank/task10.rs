use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        assert_eq!(sock_merchant(9, vec![10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
    }
}