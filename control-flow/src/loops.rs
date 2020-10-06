#[allow(dead_code)]
pub fn print_loop() {
    // loop == while true
    let mut counter = 0;
    loop {
        counter += 1;
        println!("loop: {}", counter);
        if counter > 5 {
            break;
        }
    }

    while counter > 0 {
        println!("loop: {}", counter);
        counter -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        // similar to Python
        println!("Value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number); //1-3
    }

    for number in 0..=4 {
        println!("{}", number); //0-4
    }

    for number in (0..4).rev() {
        println!("{}", number); //3-0
    }
}
