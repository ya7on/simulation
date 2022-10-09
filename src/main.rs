use simulation::ai::neural::Neural;
use simulation::field::Field;
use simulation::{HEIGHT, WIDTH};
use std::io;
use std::io::Write;

fn main() {
    let mut field = Field::new(WIDTH, HEIGHT);

    loop {
        field.step();
        field.draw();
    }
}
