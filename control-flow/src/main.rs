use std::time::Instant;

mod if_condition;
mod loops;
mod thread;

pub fn main() {
    let start = Instant::now();
    //if_condition::print_if();
    //loops::print_loop();
    thread::print_thread();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

}
