use crate::{bitboards::Bitboard, board::idx_to_file_rank, movegen::attack_vectors::AttackVector};

pub trait AVExt {
    fn gib_rook_vectors(&mut self);
    fn gib_bishop_vectors(&mut self);
}

impl AVExt for AttackVector {
    fn gib_rook_vectors(&mut self) {
        let mut i: u8 = 0;
        while i < 64 {
            let mut bb: Bitboard = 0;

            let (file, rank) = idx_to_file_rank(i);
            let index = (1 as u64) << i;

            for r in (rank+1)..8 {
                let up = (8 - r) * 8;
                bb |= index << up;
            }
            for r in 0..rank {
                let down = (rank - r) * 8;
                bb |= index >> down;
            }
            for f in (file+1)..8 {
                let right = 8 - f;
                bb |= index << right;
            }
            for f in 0..file {
                let left = file - f;
                bb |= index >> left;
            }

            self[i as usize] = bb;
            i+=1;
        }
    }
    fn gib_bishop_vectors(&mut self) {}
}