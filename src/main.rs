#[macro_use]
mod utils;
#[allow(unused)]
use utils::*;

mod current;
mod parts {
    #![allow(unused)]

    pub mod part_2024_01_1;
    pub mod part_2024_01_2;
    pub mod part_2024_01_3;
    // <INSERTION_POINT>
}

fn main() {
    current::run();
}
