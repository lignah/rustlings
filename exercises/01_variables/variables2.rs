fn main() {
    // Rust infers its type as `i32` which is the default type for integers.
    let x: u8 = b'A';

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
