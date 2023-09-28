struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut idx = 0;
        let mut str_result = String::new();

        loop {
            let target = &strs[0].chars().nth(idx);
            if target.is_none() {
                break;
            }
            for i in 1..strs.len() {
                if target.unwrap() != strs[i].chars().nth(idx).unwrap_or('\0') {
                    return str_result;
                }
            }
            str_result.push(target.unwrap());
            idx += 1;
        }

        str_result
    }
}