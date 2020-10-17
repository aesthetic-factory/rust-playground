use rand::{thread_rng, Rng};
use std::time::Instant;
use std::vec::Vec;

const SIZE: usize = 10;

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

    let mut _rand_vec = vec![0; SIZE];
    thread_rng().fill(&mut _rand_vec[..]);
    println!("rand_vec : {:?}", _rand_vec);

    vec_w_init_val.swap(0, 3);
    print_summary("vec_w_init_val after swap", &vec_w_init_val);
    vec_w_init_val.pop();
    print_summary("vec_w_init_val after pop", &vec_w_init_val);
}

fn simd() -> std::vec::Vec<f32> {
    // #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]

    // if is_x86_feature_detected!("avx2") {
    //     println!("avx2");
    // }
    let lots_of_3s = (&[-123.456f32; 4096][..])
        .iter()
        .map(|v| 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() - 4.0 - 2.0)
        .collect::<Vec<f32>>();
    lots_of_3s
}

#[allow(dead_code)]
pub fn simd_benechmark() {
    let start = Instant::now();

    for _ in 1..10000 {
        simd();
    }

    let duration = start.elapsed();
    println!("SIMD time elapsed is: {:?}", duration);
}
