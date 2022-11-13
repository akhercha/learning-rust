use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match complements.get(num) {
                Some(&index) => return vec![index, i as i32],
                None => complements.insert(target - num, i as i32),
            };
        }

        vec![]
    }

    pub fn naive_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
