mod lib;
mod cell;
use crate::cell::Cell;

fn main() {
    lib::printhello();
    //
    let mut cell_a = Cell::new();
    println!("status: {:?}, x: {}, y: {}", cell_a.get_status(), cell_a.get_x(), cell_a.get_y());
    //
    cell_a.set_x(15);
    cell_a.set_y(20);
    cell_a.change_status();
    println!("status: {:?}, x: {}, y: {}", cell_a.get_status(), cell_a.get_x(), cell_a.get_y());
}
