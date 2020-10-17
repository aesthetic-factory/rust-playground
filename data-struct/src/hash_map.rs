use std::collections::HashMap;

fn get_phone(name: &str, map: &HashMap<&str, u32>) {
    match map.get(name) {
        Some(phone_num) => println!("{}: {}", name, phone_num),
        None => println!("{}: Not Found", name),
    }
}

#[allow(dead_code)]
pub fn print_hash_map() {
    let mut phone_book: HashMap<&str, u32> = HashMap::new();
    let mut phone_book_lite: HashMap<&str, u32> = HashMap::with_capacity(1);
    phone_book.insert("John", 12345678);
    phone_book.insert("Peter", 87654321);
    phone_book.insert("Mary", 14785236);
    println!("{:?}", phone_book);

    phone_book.remove("Peter");
    println!("{:?}", phone_book);
    println!("phone book cap: {}", phone_book.capacity()); // still 3 after remove 1 item

    get_phone("John", &phone_book);
    get_phone("john", &phone_book); // error because case sensitive

    phone_book.insert("Paul", 96385274);
    get_phone("Paul", &phone_book);

    for key in phone_book.keys() {
        println!("{}", key); // loop all keys
    }
    for val in phone_book.values() {
        println!("{}", val); // loop all values
    }

    for (key, val) in phone_book.iter() {
        println!("key: {} val: {}", key, val); // loop all items
    }

    println!("phone book len: {}", phone_book.len()); // 3

    for (key, val) in phone_book.drain() {
        println!("key: {} val: {}", key, val); // loop all items and clear map
    }
    println!("phone book len: {}", phone_book.len()); // 0

    println!("phone book lite cap: {}", phone_book_lite.capacity()); // 1
    phone_book_lite.insert("John", 12345678);
    phone_book_lite.insert("Peter", 87654321);
    phone_book_lite.insert("Mary", 14785236);
    phone_book_lite.insert("John", 20000000);
    println!("{:?}", phone_book_lite);
    println!("phone book lite cap: {}", phone_book_lite.capacity()); // 3
}
