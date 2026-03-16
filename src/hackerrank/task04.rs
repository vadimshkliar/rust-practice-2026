pub fn round_grade(grade: i32) -> i32 {
    if grade < 38 {
        return grade;
    }

    let next_multiple = ((grade / 5) + 1) * 5;

    if next_multiple - grade < 3 {
        next_multiple
    } else {
        grade
    }
}

pub fn round_grades(grades: Vec<i32>) -> Vec<i32> {
    grades.into_iter().map(round_grade).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_single() {
        assert_eq!(round_grade(84), 85);
    }

    #[test]
    fn test_no_round() {
        assert_eq!(round_grade(57), 57);
    }

    #[test]
    fn test_below_38() {
        assert_eq!(round_grade(37), 37);
    }

    #[test]
    fn test_multiple_grades() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(round_grades(input), expected);
    }
}