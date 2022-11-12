use std::fs::{File, self};
use std::io::{self, Read, ErrorKind};


fn main() {
    // handle error
    // {
    //     let greeting_file_result = File::open("hello.txt");

    //     let _greeting_file = match greeting_file_result {
    //         Ok(file) => file,
    //         Err(error) => panic!("Problem opening the file: {:?}", error),
    //     };
    // }

    // matching different errors
    {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    // writing the same but with CLOSURES
    {
        let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    // shortcuts of match
    {
        // this will open hello and get the file OR panic
        let _greeting_file = File::open("hello.txt").unwrap();
    }
    {
        // can use expect also, we can send the message we want to panic!
        let _greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in the project.");
        /*
            In production-quality code, most Rustaceans choose expect rather than
            unwrap and give more context about why the operation is expected to always
            succeed.
            That way, if your assumptions are ever proven wrong, you have more
            information to use in debugging.
         */
    }
}

// Propagating Errors:
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// Shortcut for Propagating Errors:
// The '?' operator placed after a Result<> value.
fn _s_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Or the same but with chaining:
fn _s2_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// AAaaaannd even shorter!!  using fs::read_to_string
fn _s3_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// operator ? works too with Option, & will return None
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}