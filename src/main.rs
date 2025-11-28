#![allow(unused_imports)]

#[macro_use]
mod utils;
use utils::*;

mod current;
mod parts {
    #![allow(unused)]
    pub use super::*;

    pub mod part_2024_01_1;
    pub mod part_2024_01_2;
    pub mod part_2024_01_3;
    pub mod part_2024_02_1;
    pub mod part_2024_02_2;
    pub mod part_2024_02_3;
    pub mod part_2024_03_1;
    pub mod part_2024_03_2;
    pub mod part_2024_03_3;
    pub mod part_2024_04_1;
    pub mod part_2024_04_2;
    pub mod part_2024_04_3;
    // <INSERTION_POINT>
}

fn main() {
    current::run();
}
