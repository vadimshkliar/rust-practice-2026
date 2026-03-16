pub fn diagonal_difference(matrix: Vec<Vec<i32>>) -> i32 {

    let n = matrix.len();
    let mut d1 = 0;
    let mut d2 = 0;

    for i in 0..n {
        d1 += matrix[i][i];
        d2 += matrix[i][n - 1 - i];
    }

    (d1 - d2).abs()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {

        let matrix = vec![
            vec![11,2,4],
            vec![4,5,6],
            vec![10,8,-12]
        ];

        assert_eq!(diagonal_difference(matrix),15);
    }

    #[test]
    fn test_case_2() {

        let matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![9,8,9]
        ];

        assert_eq!(diagonal_difference(matrix),2);
    }

    #[test]
    fn test_case_3() {

        let matrix = vec![
            vec![5]
        ];

        assert_eq!(diagonal_difference(matrix),0);
    }
}