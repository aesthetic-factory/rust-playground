// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str

#[allow(dead_code)]
fn main() {
    let _str: &str = "Hello world";
    println!("Print &str: {:?}", _str);
    println!("Length of &str: {:?}", _str.len());

    let _string = String::from("Hello world");
    println!("Print string: {:?}", _string);
    println!("Length of _string: {:?}", _string.len());
}
