struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _guard = Guard("first");
    let _guard = Guard("second");
    println!("end");
}
