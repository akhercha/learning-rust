fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn _get_second_word(s: &String) -> (usize, usize) {
    (0, s.len())
}

fn get_first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    // Naive way
    {
        let s: String = String::from("Salut les bros c LRB");

        println!("Sentence is [{}]{}", s.len(), s);
        println!("Index of first space is {}", get_first_word(&s));
    }
    
    // Rust gigachad way : using Slices
    {
        let mut s: String = String::from("Salut les bros c LRB");
        println!("Sentence is [{}] {}", s.len(), s);
        
        let first_word = get_first_word_slice(&s);
        println!("First word is: '{first_word}'");
        s.clear();
        // println!("First word is: '{first_word}'");  <- This would cause the clear() function
        //                                                to fail cause we use a slice of s after !!
    }

    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let _word = first_word(&my_string[0..6]);
        let _word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let _word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        let _word = first_word(&my_string_literal[0..6]);
        let _word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let _word = first_word(my_string_literal);
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice_of_a = &a[1..3];
        assert_eq!(slice_of_a, &[2, 3]);
    }
}
