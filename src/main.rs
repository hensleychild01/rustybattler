mod bitboards;
mod board;
mod enums;
mod movegen;

use crate::bitboards::{Bitboard, BitboardExt};

use crate::movegen::attack_vectors::PAWN_AVECS;

fn main() {
    let i = 39;
    ((1 as Bitboard) << i).print();
    PAWN_AVECS[i].print();
}
