use std::time::Instant;

mod if_condition;
mod loops;



fn main() {
    let start = Instant::now();
    //if_condition::print_if();
    //loops::print_loop();

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
