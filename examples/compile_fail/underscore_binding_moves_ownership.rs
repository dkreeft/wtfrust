fn main() {
    let message = String::from("hello");
    let _ignored = message;
    println!("{message}"); // E0382: borrow of moved value
}
