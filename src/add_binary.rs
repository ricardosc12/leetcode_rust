struct Solution {}

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut max = std::cmp::max(a.len(), b.len()) as i32;
        let mut result = String::new();
        let mut rest = 0;
        while max > 0 {
            let a_: u32 = a.pop().unwrap_or('0').to_digit(10).unwrap();
            let b_ = b.pop().unwrap_or('0').to_digit(10).unwrap();

            if a_ + b_ + rest > 1 {
                if a_ + b_ + rest > 2 {
                    result.insert(0, '1');
                } else {
                    result.insert(0, '0');
                }
                if max == 1 {
                    result.insert(0, '1');
                }
                rest = 1;
            } else {
                result.insert(0, char::from_digit(a_ + b_ + rest, 10).unwrap());
                rest = 0;
            }
            max -= 1;
        }

        result
    }
}
