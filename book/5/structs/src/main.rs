struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        // Or this shorthand =>
        email,
        active: true,
        sing_in_count: 0,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct -> don't have any param
struct AlwaysEqual;

fn main() {
    {
        let _user: User = User {
            email: String::from("email@email.com"),
            username: String::from("user"),
            active: true,
            sing_in_count: 1,
        };
    
        // mutable =>
        let mut user: User = User {
            email: String::from("email@email.com"),
            username: String::from("user"),
            active: true,
            sing_in_count: 1,
        };
        user.email = String::from("new@email.com");
    
        // with fn
        let user1: User = build_user(String::from("email@email.com"), String::from("test"));
        let _user2: User = User {
            email: String::from("email2@email.com"),
            ..user1
        };
    }

    {
        let _black = Color(0, 0, 0);
        // they have different types even though they got same types inside
        let _origin = Point(0, 0, 0);
    }

    {
        let subject = AlwaysEqual;
    }
}
