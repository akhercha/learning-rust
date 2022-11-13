use crate::exercices::two_sum;

pub mod exercices;

fn main() {
    let target = 25;
    let input = vec![42, 52, 12, 23, 78, 13, 12];
    dbg!(two_sum::Solution::two_sum(input, target));
    let input = vec![42, 52, 12, 23, 78, 13, 12];
    dbg!(two_sum::Solution::naive_two_sum(input, target));
}
