use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn recursive_finder(
        answers: Vec<String>,
        answer: Vec<char>,
        digits: Vec<char>,
        numbers_letters_associations: HashMap<u32, &str>
    ) -> String {
        if (digits.len() == 0) && (answer.len() > 0) {
            return answer.into_iter().collect();
        }

        let possible_letters: Vec<char> = numbers_letters_associations
            .get(&(digits[0].to_digit(10).unwrap()))
            .copied()
            .unwrap_or("")
            .chars()
            .collect();

        for letter in possible_letters {
            answer.push(letter);
            digits.drain(0..1);
            answers.push(Solution::recursive_finder(answers, answer, digits, numbers_letters_associations));
        }
        String::from("")
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();
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
        let mut answers: Vec<String> = vec![];
        Solution::recursive_finder(answers, vec![], digits, numbers_letters_associations);
        answers
    }
}
