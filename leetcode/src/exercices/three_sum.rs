pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums: Vec<i32> = nums;
        nums.sort();
        let mut triplets: Vec<Vec<i32>> = vec![];
        let mut low: usize;
        let mut high: usize;
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            low = i + 1;
            high = nums.len() - 1;
            while low < high {
                if (high < nums.len() - 1) && (nums[high] == nums[high + 1]) {
                    high -= 1;
                    continue;
                }
                if nums[i] + nums[low] + nums[high] > 0 {
                    high -= 1;
                } else if nums[i] + nums[low] + nums[high] < 0 {
                    low += 1;
                } else {
                    triplets.push(vec![nums[i], nums[low], nums[high]]);
                    low += 1;
                    high -= 1;
                }
            }
        }
        triplets
    }
}
