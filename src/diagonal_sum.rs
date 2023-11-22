struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let tam = mat[0].len();
        let mut sum = 0;
        for x in 0..tam {
            for y in 0..tam {
                if x == y || x+y == tam-1{
                    sum += mat[x][y];
                }
            }
        }

        sum
    }
}