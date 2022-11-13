use std::{io, process::exit};

fn five() -> i32 {
    5
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    let f_nbr: f32 = 42.42;
    println!("{f_nbr}");

    //
    // ======= TUPLE TYPE =======
    //
    let tup: (i32, i64, u32, f32) = (500, 500, 500, 42.42);
    let (w, _x, _y, _z) = tup;
    let y: u32 = tup.2;
    println!("{}, {}, {}, {}", w, tup.0, tup.3, y);

    println!("\n");
    //
    // ======= ARRAY TYPE =======
    //
    let arr = [1, 2, 3, 4];
    println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3]);

    let arr: [i32; 5] = [3; 5]; // Says that we want the value 3 for 5 indexes (all size)
                                // Same as writing: let a: [i32; 5] = [3, 3, 3, 3, 3];
    println!("{}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    println!("Please enter the array index:");
    let mut array_index: String = String::new();
    io::stdin()
        .read_line(&mut array_index)
        .expect("Failed to read line.");

    let array_index: usize = match array_index.trim().parse() {
        Ok(res) => res,
        Err(_) => {
            println!("Must be a positive number.");
            exit(1);
        }
    };

    let element: i32 = arr[array_index];
    println!("The value at index {} is {}.", array_index, element);


    println!("\n");
    //
    // ======= FUNCTIONS =======
    //
    another_function(21, 'g');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    println!("The function returned: {}", five());


    println!("\n");
    //
    // ======= CONTROL FLOW =======
    //
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true; 
    let _number = if condition { 5 } else { 6 }; // ternary


    println!("\n");
    //
    // ======= LOOPS =======
    //
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // will return count * 2
        }
    };
    println!("Should be 20: {result}");

    let mut counter = 0;
    'counting_up: loop {
        println!("count: {counter}");
        let mut remaining = 5;

        loop {
            println!("remaining: {remaining}");
            if remaining == 4 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("Count is: {counter}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    print!("Array is [");
    while index < 5 {
        print!("{}, ", a[index]);
        index += 1;
    }
    println!("]");

    let a = [10, 20, 30, 40, 50];
    print!("Array is [");
    for element in a {
        print!("{}, ", element);
    }
    println!("]");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
