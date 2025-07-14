mod board;
mod enums;
mod bitboards;
use board::{get_empty_board};
use bitboards::{Bitboard, BitboardExt};

fn main() {
    let mut b = get_empty_board(); 
    b.init();
    b.king_bbs[1].print();
}
