#[allow(dead_code)]
#[allow(unused_imports)]

#[macro_use]

use clap::{Arg, App};
use std::cell::RefCell;

pub static mut SHUTDOWN: RefCell<u32> = RefCell::new(0);


fn main() {
    println!("Hello, world!");
}
