pub fn staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        result.push_str(&" ".repeat((n - i) as usize));
        result.push_str(&"#".repeat(i as usize));
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        let expected = "   #\n  ##\n ###\n####\n";
        assert_eq!(staircase(4), expected);
    }

    #[test]
    fn test_staircase_2() {
        let expected = " #\n##\n";
        assert_eq!(staircase(2), expected);
    }
}