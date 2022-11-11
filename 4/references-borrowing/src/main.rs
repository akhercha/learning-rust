fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_a_reference(s: &mut String) {
    s.push_str(" [WAS A SCAM]");
}

fn main() {
    {
        let s1 = String::from("bitconnectttttttttttt");
        let len = calculate_length(&s1);

        println!("Size of '{s1}' is {len}");
    }

    {
        let mut s1 = String::from("FTX");
        println!("s1 is: '{s1}'");

        change_a_reference(&mut s1);
        println!("s1 is: '{s1}'");
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // OK
        let r2 = &s; // OK
        println!("{} and {}", r1, r2);
        
        let r3 = &mut s; // Only works cause later we don't use r1 or r2.
        println!("{}", r3);
    }
}
