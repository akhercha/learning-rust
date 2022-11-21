pub mod exercices;

fn main() {
    dbg!(exercices::my_atoi::Solution::my_atoi(
        "2147483648".to_string()
    ));
}
