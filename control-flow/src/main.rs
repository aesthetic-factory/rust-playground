use std::time::Instant;

mod if_condition;
mod loops;
mod thread;
mod tokio;

pub fn main() {
    let start = Instant::now();
    //if_condition::print_if();
    //loops::print_loop();
    //thread::print_thread();
    tokio::print_tokio();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

}
