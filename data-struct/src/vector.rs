use rand::{thread_rng, Rng};
use std::vec::Vec;

const SIZE:usize = 10;

fn print_summary(label: &str, vector: &Vec<u64>) {
    println!(
        "{}:    cap: {}, len:{}, {:?}",
        label,
        vector.capacity(),
        vector.len(),
        vector
    );
}
#[allow(dead_code)]
pub fn print_vector() {
    let mut vec_w_init_val = vec![1, 2, 3];
    vec_w_init_val.push(4);

    let mut vec_w_init_size = vec![0; 5];
    vec_w_init_size.push(1);

    let mut vec_u64: Vec<u64> = Vec::new();
    vec_u64.push(1);
    vec_u64.push(2);

    let mut vec_cap: Vec<u64> = Vec::with_capacity(2); // also allocate memory

    print_summary("vec_w_init_val", &vec_w_init_val);
    print_summary("vec_w_init_size", &vec_w_init_size);
    print_summary("vec_u64", &vec_u64);
    print_summary("vec_cap", &vec_cap);


    vec_cap.push(1);
    vec_cap.push(2); // still no need to reallocate memory
    vec_cap.push(3); // execess cap, need to reallocate memory
    print_summary("vec_cap", &vec_cap);

    let mut _rand_vec= vec![0; SIZE];
    thread_rng().fill(&mut _rand_vec[..]);
    println!("rand_vec : {:?}", _rand_vec);
}
