pub fn migratory_birds(arr: Vec<i32>) -> i32 {

    let mut count = [0; 6];

    for bird in arr {
        count[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..6 {
        if count[i] > max_count {
            max_count = count[i];
            result = i as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![1,4,4,4,5,3];
        assert_eq!(migratory_birds(arr),4);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![1,1,2,2,3];
        assert_eq!(migratory_birds(arr),1);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![5,5,5,4,4];
        assert_eq!(migratory_birds(arr),5);
    }
}