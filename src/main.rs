mod board;
use board::Board;

use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_millis(1000);
    let mut board = Board::init(32, 32);
    loop {
        board.render();
        thread::sleep(one_second/15);
        board.next_generation();
        clearscreen::clear().expect("Failed to clear screen");
    }
}