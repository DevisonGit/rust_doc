enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, 
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// metodo de um enum
impl Message {
    fn call(&self) {
    }
}


fn route(ip_king: IpAddrKind){}

fn main() {
    // cria instancias
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home_v2 = IpAddrV2::V4(127, 0, 0, 1);
    let loopback_v2 = IpAddrV2::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    // enum option
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
}
