pub fn breaking_records(scores: Vec<i32>) -> (i32, i32) {

    let mut max = scores[0];
    let mut min = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {

        if score > max {
            max = score;
            max_breaks += 1;
        }

        if score < min {
            min = score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_case() {

        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

        assert_eq!(breaking_records(scores), (2, 4));
    }

    #[test]
    fn test_case_2() {

        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];

        assert_eq!(breaking_records(scores), (4, 0));
    }

    #[test]
    fn test_single_score() {

        let scores = vec![100];

        assert_eq!(breaking_records(scores), (0, 0));
    }
}