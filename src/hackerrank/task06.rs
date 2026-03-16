pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {

    if x1 == x2 {
        return "YES".to_string();
    }

    if v1 <= v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_yes_case() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_no_case() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_start() {
        assert_eq!(kangaroo(5, 2, 5, 3), "YES");
    }

    #[test]
    fn test_far_apart() {
        assert_eq!(kangaroo(0, 1, 10, 1), "NO");
    }
}