pub fn birthday_cake_candles(candles: Vec<i32>) -> i32 {

    if candles.is_empty() {
        return 0;
    }

    let mut max = candles[0];
    let mut count = 0;

    for &c in &candles {

        if c > max {
            max = c;
            count = 1;
        }
        else if c == max {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![3,2,1,3];
        assert_eq!(birthday_cake_candles(arr),2);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![4,4,1,3];
        assert_eq!(birthday_cake_candles(arr),2);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![1];
        assert_eq!(birthday_cake_candles(arr),1);
    }
}