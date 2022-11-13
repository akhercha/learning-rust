pub struct Solution {}

impl Solution {
    pub fn backtrack(
        all_permutations: &mut Vec<Vec<i32>>,
        permutation: &mut Vec<i32>,
        nums: &Vec<i32>,
    ) -> Vec<Vec<i32>> {
        if permutation.len() == nums.len() {
            all_permutations.push(permutation.clone());
        } else {
            for nb in nums {
                if permutation.contains(&nb) {
                    continue;
                }
                permutation.push(*nb);
                Solution::backtrack(all_permutations, permutation, nums);
                permutation.pop();
            }
        }
        all_permutations.clone()
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::backtrack(&mut (vec![]), &mut (vec![]), &nums)
    }
}
