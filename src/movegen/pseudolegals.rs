use crate::{
    bitboards::{Bitboard, BitboardExt},
    board::{Board, idx_from_file_rank, idx_to_file_rank},
    enums::Color,
    movegen::{
        attack_vectors::{BISHOP_AVECS, CROWNIES_AVECS, HORSEY_AVECS},
        move_rep::{Move, MoveExt, MoveList},
    },
};

pub fn gen_file_mask(file: u8) -> Bitboard {
    let mut bb = 0;
    for _ in 0..8 {
        bb |= (1 as u64) << file;
        bb <<= 8;
    }
    bb
}

pub fn gen_rank_mask(rank: u8) -> Bitboard {
    0b11111111 << rank
}

pub fn get_bishop_attacks(board: &Board, index: usize) -> Bitboard {
    let mut bb: Bitboard = 0;

    let avec = BISHOP_AVECS[index];

    let blockers = avec & board.get_occupied_squares();

    let (file, rank) = idx_to_file_rank(index as u8);

    // Southwest
    let mut limiter = std::cmp::min(file, rank);
    for i in 1..limiter + 1 {
        let f = file - i;
        let r = rank - i;
        let sq = (1 as u64) << idx_from_file_rank(f, r);
        bb |= sq;
        if sq & blockers > 0 {
            break;
        }
    }
    // Northwest
    limiter = std::cmp::min(file, 7 - rank);
    for i in 1..limiter + 1 {
        let f = file - i;
        let r = rank + i;
        let sq = (1 as u64) << idx_from_file_rank(f, r);
        bb |= sq;
        if sq & blockers > 0 {
            break;
        }
    }
    // Northeast
    limiter = std::cmp::min(7 - file, 7 - rank);
    for i in 1..limiter + 1 {
        let f = file + i;
        let r = rank + i;
        let sq = (1 as u64) << idx_from_file_rank(f, r);
        bb |= sq;
        if sq & blockers > 0 {
            break;
        }
    }
    // Southeast
    limiter = std::cmp::min(7 - file, rank);
    for i in 1..limiter + 1 {
        let f = file + i;
        let r = rank - i;
        let sq = (1 as u64) << idx_from_file_rank(f, r);
        bb |= sq;
        if sq & blockers > 0 {
            break;
        }
    }

    let mut copy = bb;

    bb
}

pub trait MoveListExt {
    fn gen_knight_moves(&mut self, board: &Board, color: Color);
    fn gen_king_moves(&mut self, board: &Board, color: Color);
    fn gen_bishop_moves(&mut self, board: &Board, color: Color);
}

impl MoveListExt for MoveList {
    fn gen_knight_moves(&mut self, board: &Board, color: Color) {
        let mut knights = board.knight_bbs[color as usize];
        let us = [board.white_bb, board.black_bb][color as usize];
        let mut from = knights.pop_lsb();
        while from > 0 {
            let mut not_blocked = HORSEY_AVECS[from] & !us;

            let mut index = not_blocked.pop_lsb();
            while index > 0 {
                let to = index;
                let m = Move::new(from as u8, to as u8);
                self.push(m);
                index = not_blocked.pop_lsb();
            }

            from = knights.pop_lsb();
        }
    }

    fn gen_king_moves(&mut self, board: &Board, color: Color) {
        let mut kings = board.king_bbs[color as usize];
        let us = [board.white_bb, board.black_bb][color as usize];

        let from = kings.pop_lsb();

        let mut not_blocked = CROWNIES_AVECS[from] & !us;

        let mut index = not_blocked.pop_lsb();
        while index > 0 {
            let to = index;
            let m = Move::new(from as u8, to as u8);
            self.push(m);
            index = not_blocked.pop_lsb();
        }
    }

    fn gen_bishop_moves(&mut self, board: &Board, color: Color) {
        let us = [board.white_bb, board.black_bb][color as usize];
        let opp = [board.black_bb, board.white_bb][color as usize];
        let mut bishops = board.bishop_bbs[color as usize];

        let mut from = 1;
        while from > 0 {
            from = bishops.pop_lsb();

            let mut moves_bb = get_bishop_attacks(board, from) & !us;
            let mut to /* stfu about the names here */ = 1;
            while to > 0 {
                to = moves_bb.pop_lsb();
                self.push(Move::new(from as u8, to as u8));
            }
        } 
    }
}
