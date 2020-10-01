// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
// https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

#[allow(dead_code)]

pub fn print_string() {
    let _str: &str = "Hello world"; // static str, hardcoded in executable
    println!("Print &str: {:?}", _str);
    println!("Length of &str: {:?}", _str.len());

    let _string = String::from("Hello world"); // heap allocated
    println!("Print string: {:?}", _string);
    println!("Length of _string: {:?}", _string.len());
}
