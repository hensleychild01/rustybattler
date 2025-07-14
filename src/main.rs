mod board;
mod enums;
mod bitboards;
use bitboards::{Bitboard, BitboardExt};

use crate::board::Board;

fn main() {
    let mut b = Board::get_empty_board(); 
    b.init();
    b.king_bbs[1].print();
}
