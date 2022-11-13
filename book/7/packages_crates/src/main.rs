use crate::garden::vegetables::{Asparagus, Tomato};
use crate::garden::fruits::{Banana, Apple};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    let fruit = Banana {};
    println!("I'm growing {:?}!", fruit);

    let plant = Tomato {};
    println!("I'm growing {:?}!", plant);

    let fruit = Apple {};
    println!("I'm growing {:?}!", fruit);
}
