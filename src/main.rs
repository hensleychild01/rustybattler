mod bitboards;
mod board;
mod enums;
mod move_generation;
mod attack_vectors;

use crate::{attack_vectors::{AVExt, AttackVector, CROWNIES_AVECS, HORSEY_AVECS}, bitboards::{Bitboard, BitboardExt}};

fn main() {
    let i = 4;
    ((1 as Bitboard) << i).print();
    CROWNIES_AVECS[i].print();
}
