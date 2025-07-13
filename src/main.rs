mod board;
mod enums;
mod bitboards;

use board::{get_empty_board};
use bitboards::{print_bitboard};

fn main() {
    let mut b = get_empty_board(); 
    b.init();
    print_bitboard(b.king_bbs[0] | b.king_bbs[1]);
}
