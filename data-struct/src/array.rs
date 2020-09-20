#[allow(dead_code)]

fn main() {
    // an array with 16 bit unsigned integer
    // fixed size
    let _num_array:  [u16; 5] = [0, 1, 2, 3, 4];

    // print whole array with ":?"
    println!("num_array : {:?}", _num_array);
    // loop for every single element in array
    // and print out element
    for n in _num_array.iter() {
        println!("Element: {}", n);
    }

    let _slice = & _num_array[0..2]; // _slice is reference of _num_array
    println!("slice : {:?}", _slice);

    // length of array
    println!("num_array length: {}", _num_array.len());
}
