// following
// https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types

#[allow(dead_code)]
pub fn print_num() {
    let _bool: bool = true;
    println!("bool : {}", _bool);

    // 4 byte
    let _char: char = 'A';
    println!("char : {}", _char);

    // 0-255
    let _unsign8: u8 = 255;
    println!("u8 : {}", _unsign8);

    // 0-65535
    let _unsign16: u16 = 65535;

    // 0-4294967295
    let _unsign32: u32 = 4294967295;

    // 0-1.84467440737e+19
    let _unsign32: u64 = 0;

    // float
    let _pi: f32 = 3.14159265359;


    const PI: f32 = 3.14159265359;
}
