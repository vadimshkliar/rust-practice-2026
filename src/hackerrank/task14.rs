pub fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) -> Result<(), i32> {

    let mut total = 0;

    for (i, &price) in bill.iter().enumerate() {
        if i != k {
            total += price;
        }
    }

    let anna_share = total / 2;

    if anna_share == b {
        Ok(())
    } else {
        Err(b - anna_share)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {

        let bill = vec![3,10,2,9];

        assert_eq!(bon_appetit(bill,1,7),Ok(()));
    }

    #[test]
    fn test_case_2() {

        let bill = vec![3,10,2,9];

        assert_eq!(bon_appetit(bill,1,12),Err(5));
    }

    #[test]
    fn test_case_3() {

        let bill = vec![5,5,5];

        assert_eq!(bon_appetit(bill,2,5),Ok(()));
    }
}