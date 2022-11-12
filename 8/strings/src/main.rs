fn main() {
    // Create a new String
    {
        let mut _s = String::new();

        let data = "initial content";
        let _s = data.to_string();

        let _s = "initial content".to_string();

        let _s = String::from("initial content");

        let s = String::from("😳😳😎😎💖🤣🤣🤣🤣");
        println!("{}", s);
    }

    // Updating a String
    {
        let mut s = String::from("foo");
        println!("{}", s);
        s.push_str(" ne passerez pas.....");
        println!("{}", s);

        let mut s1 = String::from("foo");
        println!("s1 is {}", s1);
        let s2 = "bar";
        s1.push_str(s2); // <- push str takes a string slice
        println!("s2 is {}", s2);
        println!("s1 is {}", s1);
        s1.push('l');
        println!("s1 is {}", s1);
    }

    // Concatenations
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("{}", s3);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{}", s);

        // OR

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }

    // Indexing
    {
        let hello = "Здравствуйте";
        // we can't index like hello[0]; check docs
        // https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings
        let s = &hello[0..4]; // but we can like this.. but be cautious
        println!("{s}");
    }

    // Iterate
    {
        for (i, c) in "Зд".chars().enumerate() {
            if i != "Зд".chars().count() - 1 {
                print!("{} ", c);
            } else {
                println!("{} ", c);
            }
        }
        for (i, b) in "Зд".bytes().enumerate() {
            if i != "Зд".bytes().len() - 1 {
                print!("{} ", b);
            } else {
                println!("{} ", b);
            }
        }
    }
}
