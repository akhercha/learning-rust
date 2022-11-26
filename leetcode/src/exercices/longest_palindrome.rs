pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;

        if s.len() == 0 {
            return "".to_string();
        }

        for i in 0..s.len() {
            let mut left = i;
            let mut right = i;

            while (right < s.len() - 1) && (s[right] == s[right + 1]) {
                right += 1;
            }

            while (left > 0) && (right + 1  < s.len()) && (s[right + 1] == s[left - 1]) {
                left -= 1;
                right += 1
            }
            if (right > left) && (right as i32 - left as i32) > (end as i32 - start as i32) {
                start = left;
                end = right;
            }
        }
        println!("start: {}, end: {}", start, end);
        s[start..=end].iter().collect()
    }
}