#[allow(dead_code)]
pub fn print_tuple() {
    let _tuple = ("hello", 123, "world", [1, 2, 3]);
    println!("First element is {}", _tuple.0);
    println!("Second element is {}", _tuple.1);
    println!("Third element is {}", _tuple.2);
    let mut counter = 0;
    for x in _tuple.3.iter() {
        println!("Element {} of the fourth element is {}", counter, x);
        counter += 1;
    }
}
