use crate::{
    bitboards::{Bitboard, BitboardExt},
    board::Board,
    enums::Color,
    movegen::{
        attack_vectors::{HORSEY_AVECS, CROWNIES_AVECS},
        move_rep::{Move, MoveList, MoveExt},
    },
};

pub trait MoveListExt {
    fn gen_knight_moves(&mut self, board: &Board, color: Color);
    fn gen_king_moves(&mut self, board: &Board, color: Color);
}

impl MoveListExt for MoveList {
    fn gen_knight_moves(&mut self, board: &Board, color: Color) {
        let mut knights = (*board).knight_bbs[color as usize];
        let us = if color == Color::White {
            (*board).white_bb
        } else {
            (*board).black_bb
        };
        let mut from = knights.pop_lsb();
        while from > 0 {
            let mut not_blocked = HORSEY_AVECS[from] & !us;

            let mut index = not_blocked.pop_lsb();
            while index > 0 {
                let to = index; 
                let m = Move::new(from as u8, to as u8);
                (*self).push(m);
                index = not_blocked.pop_lsb();
            }

            from = knights.pop_lsb();
        }
    }

    fn gen_king_moves(&mut self, board: &Board, color: Color) {
        let mut kings = (*board).king_bbs[color as usize];
        let us = if color == Color::White {
            (*board).white_bb
        } else {
            (*board).black_bb
        };
        let mut from = kings.pop_lsb();

        let mut not_blocked = CROWNIES_AVECS[from] & !us;

        let mut indices: Vec<u8> = vec![];
        let mut index = not_blocked.pop_lsb();
        while index > 0 {
            indices.push(index as u8);
            index = not_blocked.pop_lsb();
        }

        for to in indices {
            // now we turn the bitboard into moves
            let m: Move = (from as u32) | ((to as u32) << 6);
            (*self).push(m);
        }
    }
}
