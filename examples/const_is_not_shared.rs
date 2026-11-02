use std::sync::atomic::{AtomicU8, Ordering};

const COUNTER: AtomicU8 = AtomicU8::new(1);

#[expect(const_item_interior_mutations)]
fn main() {
    COUNTER.store(2, Ordering::Relaxed);
    println!("{}", COUNTER.load(Ordering::Relaxed));
}
