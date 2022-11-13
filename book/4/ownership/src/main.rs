fn makes_copy(x: i32) {
    println!("[MAKES_COPY] This is just a copy :( -> {x}");
}

fn takes_ownership(some_string: String) -> String {
    println!("[TAKES_OWNERSHIP] Ahah, now I own {some_string} !!");
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    {
        let mut s: String = String::from("Hello");
        s.push_str(", world !");
        println!("{s}");
    }

    {
        let x = 5;
        let _y = x; // Simple type so _y is a copy of x => 5!

        let mut s1 = String::from("hello");
        let s2 = s1.clone();
        s1.push_str(", world !");
        println!("s1: {s1}");
        println!("s2: {s2}");
    }

    {
        let mut s = String::from("hello");
        s = takes_ownership(s);
        println!("[MAIN] Hehe I took it back: {s}");

        let x = 5;
        makes_copy(x);
        println!("[MAIN] Hehe yea I still got it: {x}");
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{s2}' is {len}");
    }
    
}
