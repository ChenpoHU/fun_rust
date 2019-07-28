use std::mem;

struct Point{
        x:f64,
        y:f64
}

fn original() -> Point{
    Point{x: 1.0, y: 1.0 }
}

fn main(){
    let p1 = original();
    let p2 = Box::new(original());
    println!{"size of p1 in stack is {} bytes", mem::size_of_val(&p1)};
    println!{"size of p2 in heap is {} bytes", mem::size_of_val(&p2)};
    let p3 = Point{x:0.1, y:0.2};
    println!{"size of p3 in heap is {} bytes, x is {}", mem::size_of_val(&p3), p3.x};
    let p4 = p1;
    println!{"size of p4 in heap is {} bytes, x is {}", mem::size_of_val(&p4), p4.x};
    let p5 = *p2;
    println!{"size of p5 in heap is {} bytes, x is {}", mem::size_of_val(&p5), p5.x};
    /*
    Assignment and copy constructors similarly don't exist because move semantics are the only semantics in Rust. At most x = y just moves the bits of y into the x variable. 
    let p5 = p2; //cannot use after partial move
    println!{"size of p5 in heap is {} bytes, x is {}", mem::size_of_val(&p5), p5.x};*/
}