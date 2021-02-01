use std::time::Instant;
use std::thread::{spawn,JoinHandle};

fn fibonacci(input: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..input {
        let t = a + b;
        a = b;
        b = t;
    }
    return b;
}

fn fibonacci_thread(input: u64) -> JoinHandle<u64>{
    let t = spawn(move || {
        return fibonacci(input);
    });
    return t;
}

#[allow(dead_code)]
pub fn print_thread() {
    let start_st = Instant::now();
    fibonacci(300);
    fibonacci(300);
    fibonacci(300);
    fibonacci(300);
    fibonacci(300);
    let duration_st = start_st.elapsed();
    println!("Single Thread is: {:?}", duration_st);


    let start_mt = Instant::now();
    let t1 = fibonacci_thread(300);
    let t2 = fibonacci_thread(300);
    let t3 = fibonacci_thread(300);
    let t4 = fibonacci_thread(300);
    let t5 = fibonacci_thread(300);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
    let duration_mt = start_mt.elapsed();
    println!("Multi Thread is: {:?}", duration_mt);
}
