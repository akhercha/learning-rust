pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
        let mut longest_prefix: Vec<char> = vec![];
        let mut i: usize = 0;
        let mut current_char: char = '0';
        'solve: loop {
            for j in 0..strs.len() {
                if j == 0 {
                    if (i >= strs[j].len()) || (strs[j].len() == 0) {
                        break 'solve;
                    }
                    current_char = strs[j][i];
                    continue;
                }
                if (i >= strs[j].len()) || (strs[j][i] != current_char) {
                    break 'solve;
                }
            }
            longest_prefix.push(current_char);
            i += 1;
        }
        longest_prefix.into_iter().collect()
    }
}
