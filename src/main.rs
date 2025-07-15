mod bitboards;
mod board;
mod enums;
mod move_generation;
mod attack_vectors;

use crate::{attack_vectors::{AVExt, AttackVector}, bitboards::{Bitboard, BitboardExt}};

fn main() {
    let mut horseys: AttackVector = [0; 64];
    // let mut crownies: AttackVector = [0; 64];
    horseys.gib_horsey_avecs();
    // crownies.gib_crown_avecs();
    println!("{:?}\n\n", horseys /*, crownies*/);
    let i = 39 as u8;
    ((1 as Bitboard) << i).print();
    horseys[i as usize].print(); 
}
