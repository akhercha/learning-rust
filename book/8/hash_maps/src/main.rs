use std::collections::HashMap;

fn main() {
    // Create an Hashmap
    {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 5);
        dbg!(&scores);
    }

    // Accessing values
    {
        // Declaration
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 5);

        // Create
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(-1);
        //    Option<&V>
        //          copied() get Option<i32>
        //          rather than Option<&i32>
        //                            unwrap_or set score to 0 if not entry
        println!("Score of {} is {}", team_name, score);

        // Iterate
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    // Updating
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        println!("{:?}", scores);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);
    }

    // Add a new pair only key is not present
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // add a new Yellow entry
        scores.entry(String::from("Yellow")).or_insert(50);
        // will not change cause Blue already exists
        scores.entry(String::from("Blue")).or_insert(50);
        dbg!(&scores);
    }

    // Updating a value based on old value
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let ref_count = map.entry(word).or_insert(0);
            *ref_count += 1;
        }
        println!("{:?}", map);
    }

    // Generate a dict with loop
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            map.entry(word).or_insert(word.to_string().chars().count());
        }
        println!("{:?}", map);
    }
}
