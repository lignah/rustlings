fn main() {
    // Reading uninitialized variables isn't allowed in Rust
    let x: i32 = b'e'.into();

    // Number 101
    println!("Number {x}");
}
