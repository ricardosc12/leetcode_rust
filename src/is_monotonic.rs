struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let tam = nums.len();
        if tam < 3 {
            return true;
        }
        for i in 0..tam - 1 {
            if nums[i] > nums[i + 1] {
                for x in (i + 1)..tam - 1 {
                    if nums[x] < nums[x + 1] {
                        return false;
                    }
                }
                return true;
            } 
            else if nums[i] < nums[i + 1] {
                for x in (i + 1)..tam - 1 {
                    if nums[x] > nums[x + 1] {
                        return false;
                    }
                }
                return true;
            }
        }

        return true;
    }
}
