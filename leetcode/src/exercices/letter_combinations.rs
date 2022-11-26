use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();
        let mut answers: Vec<String> = vec![];
        let mut possible_letters: Vec<char>;
        let numbers_letters_associations: HashMap<u32, &str> = [
            (2, "abc"),
            (3, "def"),
            (4, "ghi"),
            (5, "jkl"),
            (6, "mno"),
            (7, "pqrs"),
            (8, "tuv"),
            (9, "wxyz"),
        ]
        .into_iter()
        .collect();
        for (digit_pos, digit) in digits.iter().enumerate() {
            possible_letters = numbers_letters_associations
                .get(&(digit.to_digit(10).unwrap()))
                .copied()
                .unwrap_or("")
                .chars()
                .collect();
        }
        answers
    }
}
