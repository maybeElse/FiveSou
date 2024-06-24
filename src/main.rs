#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod tiles;
mod yaku;
mod errors;
mod scoring;

use crate::tiles::{Tile, SimpleTile, HonorTile, DragonTile, WindTile, Suit};
use crate::yaku::{Yaku, YakuSpecial, WinType};

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_validity(){
    }

    #[test]
    fn yaku_counts(){
    }

    #[test]
    fn han_counts(){
    }

    #[test]
    fn hand_scoring(){
    }
}