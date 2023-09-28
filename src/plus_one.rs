struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut idx = digits.len() - 1;

        loop {
            if digits[idx] + 1 > 9 {
                if idx == 0 {
                    digits[idx] = 0;
                    digits.insert(0, 1);
                    return digits;
                }
                digits[idx] = 0;
                idx -= 1;
            } else {
                digits[idx] += 1;
                break;
            }
        }

        digits
    }
}