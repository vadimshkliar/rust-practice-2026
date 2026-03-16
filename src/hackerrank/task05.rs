pub fn count_apples_and_oranges(
    s: i64,
    t: i64,
    a: i64,
    b: i64,
    apples: Vec<i64>,
    oranges: Vec<i64>,
) -> (i32, i32) {

    let mut apple_count = 0;
    let mut orange_count = 0;

    for d in apples {
        let position = a + d;
        if position >= s && position <= t {
            apple_count += 1;
        }
    }

    for d in oranges {
        let position = b + d;
        if position >= s && position <= t {
            orange_count += 1;
        }
    }

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let s = 7;
        let t = 11;

        let a = 5;
        let b = 15;

        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        assert_eq!(
            count_apples_and_oranges(s, t, a, b, apples, oranges),
            (1, 1)
        );
    }

    #[test]
    fn test_no_fruits_on_house() {

        let s = 2;
        let t = 3;

        let a = 1;
        let b = 5;

        let apples = vec![-1];
        let oranges = vec![1];

        assert_eq!(
            count_apples_and_oranges(s, t, a, b, apples, oranges),
            (0, 0)
        );
    }

    #[test]
    fn test_all_apples_inside() {

        let s = 2;
        let t = 10;

        let a = 4;
        let b = 20;

        let apples = vec![0, 1, 2];
        let oranges = vec![5];

        assert_eq!(
            count_apples_and_oranges(s, t, a, b, apples, oranges),
            (3, 0)
        );
    }
}