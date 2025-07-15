mod bitboards;
mod board;
mod enums;
mod move_generation;
mod attack_vectors;

use crate::{attack_vectors::{AVExt, AttackVector, HORSEY_AVECS}, bitboards::{Bitboard, BitboardExt}};

fn main() {
    let i = 32;
    ((1 as Bitboard) << i).print();
    HORSEY_AVECS[i as usize].print(); 
}
