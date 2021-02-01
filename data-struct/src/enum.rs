use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[allow(dead_code, unused_variables)]

fn complex_enum() {
    #[derive(Debug, PartialEq)]
    enum IpAddrKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    #[derive(Debug, PartialEq)]
    enum IpAddrS {
        V4(String),
        V6(String),
    }
    fn compare_adr_kind(addr1: &IpAddr, addr2: &IpAddr) -> bool {
        if addr1.kind == addr2.kind {
            return true;
        }
        return false;
    }
    fn compare_adr_kind_s(addr1: &IpAddrS, addr2: &IpAddrS) -> bool {
        if addr1 == addr2 {
            return true;
        }
        return false;
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback1 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let loopback2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::2"),
    };

    let home_s = IpAddrS::V4(String::from("127.0.0.1"));

    let loopback_s1 = IpAddrS::V6(String::from("::1"));
    let loopback_s2 = IpAddrS::V6(String::from("::2"));

    println!("{:?}", home);
    println!("{:?}", loopback1);
    println!("Equals: {}", compare_adr_kind(&loopback1, &loopback2));
    println!("Equals: {}", compare_adr_kind(&loopback1, &home));

    println!("{:?}", home_s);
    println!("{:?}", loopback_s1);

    println!("Equals: {}", compare_adr_kind_s(&loopback_s1, &loopback_s1));
    println!("Equals: {}", compare_adr_kind_s(&loopback_s1, &loopback_s2));

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // method body would be defined here
            match self{
                Message::Write(string) => println!("Write: {}", string),
                _=>{}
            }

        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn simple_enum() {
    #[derive(Debug, FromPrimitive)]
    enum Code {
        NoError = 0,
        Timeout,
        SomeOtherError = 10,
        UnknownError=100,
    }

    fn map_to_code(code: u32)-> Code{
        match FromPrimitive::from_u32(code) {
            Some(Code::NoError) => Code::NoError,
            Some(Code::Timeout) => Code::Timeout,
            Some(Code::SomeOtherError) => Code::SomeOtherError,
            Some(Code::UnknownError) => Code::UnknownError,
            _            => Code::UnknownError
        }
    }

    let code_1 = Code::NoError;
    let code_2 = Code::Timeout;
    let code_3 = Code::SomeOtherError;

    let code_4: Code = map_to_code(10);
    let code_5: Code = map_to_code(105);

    println!("code_1 {:?}", code_1 as usize);
    println!("code_2 {:?}", code_2 as usize);
    println!("code_3 {:?}", code_3);
    println!("code_4 {:?}", code_4);
    println!("code_5 {:?}", code_5);
}

pub fn print_enum() {
    simple_enum();
    complex_enum();
}
