use core::num;

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut aux: i32;
        for i in 0..nums.len() {
            if nums[i] % 2 != 0 {
                for j in i..nums.len() {
                    if nums[j] % 2 == 0 {
                        aux = nums[j];
                        nums[j] = nums[i];
                        nums[i] = aux;
                    }
                }
            }
        }
        nums
    }
}

fn main() {
    let nums = vec![0,2,1];
    let index = Solution::sort_array_by_parity(nums);

    // let num = &nums[1..3];

    println!("Solution: {index:?}");
}
