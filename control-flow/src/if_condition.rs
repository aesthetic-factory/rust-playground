const GLOBAL_NUM: u8 = 10;

#[allow(dead_code)]
pub fn print_if() {
    let local_num = 2;
    if local_num < GLOBAL_NUM {
        println!("Local number is samller");
    }

    let one_line_define = if local_num < GLOBAL_NUM { true } else { false };
    println!("Bool: {}", one_line_define);
}
