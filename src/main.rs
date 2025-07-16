mod bitboards;
mod board;
mod enums;
mod movegen;

use crate::bitboards::{Bitboard, BitboardExt};

use crate::movegen::attack_vectors::{AttackVector, PAWN_AVECS, ROOK_AVECS};
use crate::movegen::sliders::AVExt;

fn main() {
    let i = 21;
    ((1 as Bitboard) << i).print();
    ROOK_AVECS[i].print();
}
