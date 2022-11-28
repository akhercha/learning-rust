use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    fn recursive_finder(
        answers: &mut Vec<String>,
        answer: &mut Vec<char>,
        digits: &mut Vec<char>,
        numbers_letters_associations: &HashMap<u32, &str>
    ) -> String {
        let mut to_add: String;
        if digits.len() == 0 {
            if answer.len() > 0 {
                return answer.iter().collect();
            } else {
                return String::from("");
            }
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
            to_add = Solution::recursive_finder(answers, &mut answer.to_vec(), &mut digits.to_vec(), numbers_letters_associations);
            if to_add.chars().count() > 0 {
                answers.push(to_add);
            }
        }
        String::from("")
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut digits: Vec<char> = digits.chars().collect();
        let mut answers: Vec<String> = vec![];
        let numbers_letters_associations: HashMap<u32, &str> = [
            (2, "abc"),
            (3, "def"),
            (4, "ghi"),
            (5, "jkl"),
            (6, "mno"),
            (7, "pqrs"),
            (8, "tuv"),
            (9, "wxyz"),
        ].iter().cloned().collect();
        Solution::recursive_finder(&mut answers, &mut (vec![]), &mut digits, &numbers_letters_associations);
        answers
    }
}
