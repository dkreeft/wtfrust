use std::{cell::Cell, rc::Rc};

fn main() {
    let cells = vec![Rc::new(Cell::new(0)); 3];
    cells[0].set(7);
    println!("{}", cells[2].get());
}
