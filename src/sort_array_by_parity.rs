struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut even_index = 0;
    
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                nums.swap(i, even_index);
                even_index += 1;
            }
        }

        nums
    }
}