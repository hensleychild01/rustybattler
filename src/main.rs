mod bitboards;
mod board;
mod enums;
mod move_generation;
mod attack_vectors;

use crate::{attack_vectors::{AVExt, AttackVector, CROWNIES_AVECS, HORSEY_AVECS, PAWN_AVECS}, bitboards::{Bitboard, BitboardExt}};

fn main() {
    
    let i = 39;
    ((1 as Bitboard) << i).print();
    PAWN_AVECS[i].print();
}
