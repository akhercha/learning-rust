use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    '_game_loop: loop {
        print!("Please input your guess: ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess.");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" is not a number.", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! "),
            Ordering::Greater => println!("Too big! "),
            Ordering::Equal => {
                println!("Gg, you got it!!!");
                break;
            }
        }
    }
}
