enum IpAddrKindBasis {
    V4,
    V6,
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn _route(_ip_kind: IpAddrKindBasis) -> i32 {
    0
}

#[derive(Debug)]
enum Message {
    _Quit,                       // no data associated, like a unit struct
    _Move { x: i32, y: i32 },    // named fields like a struct
    Write(String),               // includes juste a single string
    _ChangeColor(i32, i32, i32), // unnamed fields
}

// We can also implements within an enum just like struct
impl Message {
    fn call(&self) {
        if let Message::Write(str) = self {
            println!("The text was {}", str);
        }
    }
}

fn main() {
    {
        let _four = IpAddrKindBasis::V4;
        let _six = IpAddrKindBasis::V6;
    }

    {
        let _home = IpAddrKind::V4(String::from("127.0.0.1"));
        let _loopback = IpAddrKind::V6(String::from("::1"));
    }

    {
        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let _some_number: Option<i32> = Some(5);
        let _some_char = Some('e');

        let _absent_number: Option<i32> = None;
    }
}
