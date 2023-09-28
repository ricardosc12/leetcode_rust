struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let tam = nums.len();
        for idx in 0..tam - 1 {
            for idy in (idx+1)..tam {
                if nums[idx] + nums[idy] == target {
                    return vec![idx as i32, idy as i32];
                }
            }
        }
        vec![0, 0]
    }
}