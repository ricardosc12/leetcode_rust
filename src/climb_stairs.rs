struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        
        let (mut prev, mut curr) = (1, 2);

        for _ in 3..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
    
        curr
    }
}