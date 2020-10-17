use rand::{thread_rng, Rng};

const SIZE: usize = 10;

#[allow(dead_code)]
pub fn print_array() {
    // an array with 16 bit unsigned integer
    // fixed size
    let _num_array: [u16; 5] = [0, 1, 2, 3, 4];

    // All elements can be initialized to the same value
    let _zeros: [i32; 500] = [0; 500];

    // print whole array with ":?"
    println!("num_array : {:?}", _num_array);
    // loop for every single element in array
    // and print out element
    for n in _num_array.iter() {
        println!("Element: {}", n);
    }

    let _slice = &_num_array[0..2]; // _slice is reference of _num_array
    println!("slice : {:?}", _slice);

    // length of array
    println!("num_array length: {}", _num_array.len());

    let _tup_array: [(u16, &str); 2] = [(0, "NO_ERROR"), (1, "ERROR")];
    // print whole array with ":?"
    println!("tup_array : {:?}", _tup_array);

    let mut _rand_array: [u32; SIZE] = [0; SIZE];
    thread_rng().fill(&mut _rand_array[..]);
    println!("rand_array : {:?}", _rand_array);
}
