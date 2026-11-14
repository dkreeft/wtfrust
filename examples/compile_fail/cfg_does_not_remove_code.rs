fn main() {
    if cfg!(any()) {
        nonexistent_function();
    }

    println!("Hello");
}