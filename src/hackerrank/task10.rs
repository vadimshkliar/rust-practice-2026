pub fn sock_merchant(ar: Vec<i32>) -> i32 {

    let mut count = [0; 101];

    for sock in ar {
        count[sock as usize] += 1;
    }

    let mut pairs = 0;

    for c in count.iter() {
        pairs += c / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![10,20,20,10,10,30,50,10,20];
        assert_eq!(sock_merchant(arr),3);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![1,1,1,1];
        assert_eq!(sock_merchant(arr),2);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![1,2,3];
        assert_eq!(sock_merchant(arr),0);
    }
}