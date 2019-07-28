use std::mem;

fn main() {
    //signed -128...127
    //u8 for unsigned 0...255
    let mut a:i8 = -128;
    a = 127;
    println!("a= {}", a);

    let ss:usize = 255;
    let size_of_ss = mem::size_of_val(&ss);
    println!("size of {}, takes {} bytes, {}-bit OS", ss, size_of_ss, size_of_ss * 8);
}
