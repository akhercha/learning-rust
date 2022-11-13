fn main() {
    // Create vectors
    {
        let _v: Vec<i32> = Vec::new();
        // or with macro
        let _v = vec!["salut", "les", "bros", "c", "lrb"];
    }

    // Update vectors
    {
        // must be mutable to change its values
        let mut v = Vec::new();
        v.push(5);
        v.push(10);
        v.push(15);
    }

    // Access values
    {
        let v: Vec<i32> = (1..=5).collect();

        // Can crash if list is too smaLL...
        let third_element: i32 = v[2];
        println!("The third element is {third_element}.");
        println!("The third element is {}.", v[2]);

        // Can crash if list is too smaLL...
        let third_element = &v[2];
        // v.push(10);  crash !!! because mutable borrow occurs
        println!("The third element is {third_element}.");

        // Never crashes!!
        let third_element: Option<&i32> = v.get(2);
        match third_element {
            Some(third_element) => println!("The third element is {third_element}."),
            None => println!("There is no third element."),
        }
    }

    // Iterate
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }

    // Use Enums to store multiple types
    // We can also use TRAITS, see later
    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(3.42),
            SpreadsheetCell::Text(String::from("Hello")),
        ];
        dbg!(&row);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
