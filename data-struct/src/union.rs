#[allow(dead_code)]

union Number {
    u: u32,
    f: f32,
}
fn print_val(num: Number){
    unsafe{
        match num{
            Number{u:1}=>{
                println!("u: {}", num.u);
            }
            Number{f:_}=>{
                println!("f: {}", num.f);
            }
        }
    }
}
pub fn print_union() {
    let num = Number { f: 2.0 };
    print_val(num);
}   
