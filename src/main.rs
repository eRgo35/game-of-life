mod args;
mod board;

use args::Args;
use board::Board;

use clap::Parser;
use std::{thread, time};

fn main() {
    let args = Args::parse();
    let mut board: Board;

    if !args.load.is_empty() {
        board = Board::load(args.load);
    } else {
        board = Board::init(args.width, args.height, args.probability, args.seed);
    }
    
    if !args.save.is_empty() {
        board.save(args.save);
    }
    
    loop {
        clearscreen::clear().expect("Failed to clear screen");
        board.render();
        thread::sleep(time::Duration::from_millis(1000 / args.tickrate));
        board.next_generation(args.overpopulation, args.underpopulation, args.repopulation);
    }
}