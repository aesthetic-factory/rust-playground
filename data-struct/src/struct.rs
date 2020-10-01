

#[derive(Debug)]
struct User {
    username: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
pub fn print_struct() {
    let _john = User{username:String::from("John"),sign_in_count:0, active:true};
    let _paul = User{username:String::from("Paul"),sign_in_count:0, active:false};
    println!("{:?}", _john);
    println!("{:?}", _paul);
}   
