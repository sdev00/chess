// #[macro_use]
extern crate num_derive;
mod chess;
use chess::*;

fn main() {
    let board = Board::default();
    println!("{}", board);
}
