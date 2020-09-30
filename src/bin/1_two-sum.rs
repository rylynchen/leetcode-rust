use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for (i, val) in nums.iter().enumerate() {
            let m = target - val;
            match map.get(&m) {
                Some(n) => {
                    return vec![n.clone(), i as i32];
                },
                None => {
                    map.insert(val.clone(), i.clone() as i32);
                }
            }
        }
        return vec![];
    }
}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let vec = Solution::two_sum(nums, target);
    println!("{:?}", vec);
}