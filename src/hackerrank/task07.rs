pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {

    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    let mut count = 0;

    for x in start..=end {

        if a.iter().all(|&v| x % v == 0) &&
           b.iter().all(|&v| v % x == 0) {

            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_case() {

        let a = vec![2,4];
        let b = vec![16,32,96];

        assert_eq!(get_total_x(a,b),3);
    }

    #[test]
    fn test_simple_case() {

        let a = vec![1];
        let b = vec![100];

        assert_eq!(get_total_x(a,b),9);
    }

    #[test]
    fn test_no_numbers() {

        let a = vec![3];
        let b = vec![4];

        assert_eq!(get_total_x(a,b),0);
    }
}