pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 { return "YES".to_string(); }
        else { return "NO".to_string(); }
    }

    let n = (x2 - x1) as f32 / (v1 - v2) as f32;

    if n >= 0.0 && n.fract() == 0.0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
        assert_eq!(kangaroo(2, 1, 1, 2), "YES");
    }
}