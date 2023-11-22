struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut direction = 0;
        let mut col_tam = matrix[0].len() - 1;
        let mut line_tam = matrix.len() - 1;
        let mut line_min = 0;
        let mut col_min = 0;
        let items = (line_tam + 1) * (col_tam + 1);
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        let mut result: Vec<i32> = vec![];
        while count != items {
            result.push(matrix[i][j]);
            if direction == 0 {
                if j == col_tam {
                    i += 1;
                    col_tam -= 1;
                    direction = 1;
                } else {
                    j += 1;
                }
                count += 1;
            } else if direction == 1 {
                if i == line_tam {
                    j -= 1;
                    line_tam -= 1;
                    direction = 2;
                } else {
                    i += 1;
                }
                count += 1;
            } else if direction == 2 {
                if j == col_min {
                    i -= 1;
                    line_min += 1;
                    direction = 3;
                } else {
                    j -= 1;
                }
                count += 1;
            } else if direction == 3 {
                if i == line_min {
                    j += 1;
                    col_min += 1;
                    direction = 0;
                } else {
                    i -= 1;
                }
                count += 1;
            }
        }

        result
    }
}

fn main() {
    let mat: Vec<Vec<i32>> = vec![[1, 2, 3].to_vec(), [4, 5, 6].to_vec()];

    let index = Solution::spiral_order(mat);

    // let num = &nums[1..3];

    println!("Solution: {index:?}");
}
