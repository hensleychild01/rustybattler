use crate::{bitboards::{Bitboard, BitboardExt}, board::idx_to_file_rank};

pub type AttackVector = [Bitboard; 64];

pub trait AVExt {
    fn gib_horsey_avecs(&mut self);
    fn gib_crown_avecs(&mut self);
}

impl AVExt for AttackVector {
    fn gib_horsey_avecs(&mut self) {
        let mut i = 0 as u8;
        while i < 64 {
            let mut bb = 0 as Bitboard; 

            let (file, rank) = idx_to_file_rank(i);
            
            let index = (1 as u64) << i;
            
            // offsets: 6, 10, 15, 17
            if rank != 0 {
                if file > 1 {
                    bb |= index >> 10;
                }
                if file < 6 {
                    bb |= index >> 6;
                }
            }
            if rank != 7 {
                if file > 1 {
                    bb |= index << 6;
                }
                if file < 6 {
                    bb |= index << 10;
                }
            }
            if file != 0 {
                if rank > 1 {
                    bb |= index >> 17;
                }
                if rank < 6 {
                    bb |= index >> 15;
                }
            }
            if file != 7 {
                if rank > 1 {
                    bb |= index << 15;
                }
                if rank < 6 {
                    bb |= index << 17;
                }
            }
            self[i as usize] = bb;

            i += 1;
        }
    }

    fn gib_crown_avecs(&mut self) {
        let mut i = 0 as u8; 
        while i < 64 {

            i += 1;
        }
    }
}

