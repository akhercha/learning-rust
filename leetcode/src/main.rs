pub mod exercices;

fn main() {
    {
        let target = 25;
        let input = vec![42, 52, 12, 23, 78, 13, 12];
        dbg!(exercices::two_sum::Solution::two_sum(input, target));
        let input = vec![42, 52, 12, 23, 78, 13, 12];
        dbg!(exercices::two_sum::Solution::naive_two_sum(input, target));
    }

    {
        let input = vec![1, 2, 3];
        dbg!(exercices::permutations::Solution::permute(input));
        let input = vec![0, 1];
        dbg!(exercices::permutations::Solution::permute(input));
        let input = vec![1];
        dbg!(exercices::permutations::Solution::permute(input));
    }
}
