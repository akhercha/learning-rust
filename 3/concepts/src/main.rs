use std::{io, process::exit};

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

    let array_index: usize = match array_index
        .trim()
        .parse() {
            Ok(res) => res,
            Err(_) => {
                println!("Must be a positive number.");
                exit(1);
            }
        };

    let element: i32 = arr[array_index];
    println!("The value at index {} is {}.", array_index, element);
}
