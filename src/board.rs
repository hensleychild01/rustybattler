use crate::bitboards::{Bitboard, BitboardExt};
use crate::enums::{Color, PieceType};
use crate::movegen::attack_vectors::{
    BISHOP_AVECS, CROWNIES_AVECS, HORSEY_AVECS, PAWN_AVECS, ROOK_AVECS,
};
use crate::movegen::pseudolegals::gen_file_mask;

#[derive(Copy, Clone)]
pub struct Square {
    pub piece: PieceType,
    pub color: Color,
}

pub fn idx_to_file_rank(idx: u8) -> (u8, u8) {
    let file = (idx as u8) & (7 as u8);
    let rank = (idx as u8) >> 3;
    (file, rank)
}

pub fn idx_from_file_rank(file: u8, rank: u8) -> u8 {
    rank << 3 | file
}

#[derive(Clone, Copy)]
pub struct Board {
    pub wtm: bool,

    pub reversables: i32,
    pub halfmove_clock: i32,
    pub castling_rights: u8,

    pub pawn_bbs: [Bitboard; 2],
    pub knight_bbs: [Bitboard; 2],
    pub bishop_bbs: [Bitboard; 2],
    pub rook_bbs: [Bitboard; 2],
    pub queen_bbs: [Bitboard; 2],
    pub king_bbs: [Bitboard; 2],

    pub mailbox: [Square; 64],

    pub white_bb: Bitboard,
    pub black_bb: Bitboard,
}

pub static STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// a1-h8
impl Board {
    pub fn get_empty_board() -> Self {
        return Self {
            wtm: true,
            reversables: 0,
            halfmove_clock: 0,
            castling_rights: 0b1111,
            pawn_bbs: [0; 2],
            knight_bbs: [0; 2],
            bishop_bbs: [0; 2],
            rook_bbs: [0; 2],
            queen_bbs: [0; 2],
            king_bbs: [0; 2],
            mailbox: [Square {
                piece: PieceType::None,
                color: Color::None,
            }; 64],
            white_bb: 0,
            black_bb: 0,
        };
    }

    pub fn init(&mut self) {
        self.load_fen(STARTING_FEN);
    }

    pub fn add_piece(&mut self, color: Color, piece: PieceType, index: usize) {
        let is_white = color == Color::White;
        let color_idx = color as usize;

        let bb = (1 as u64) << index;

        if is_white {
            self.white_bb |= bb;
        } else {
            self.black_bb |= bb;
        }

        self.mailbox[index].color = color;

        match piece {
            PieceType::Pawn => {
                self.pawn_bbs[color_idx] |= bb;
            }
            PieceType::Knight => {
                self.knight_bbs[color_idx] |= bb;
            }
            PieceType::Bishop => {
                self.bishop_bbs[color_idx] |= bb;
            }
            PieceType::Rook => {
                self.rook_bbs[color_idx] |= bb;
            }
            PieceType::Queen => {
                self.queen_bbs[color_idx] |= bb;
            }
            PieceType::King => {
                self.king_bbs[color_idx] |= bb;
            }
            _ => {}
        }
    }

    pub fn remove_piece(&mut self, index: usize) -> (Color, PieceType) {
        let color = self.mailbox[index].color;
        let piece = self.mailbox[index].piece;

        let bb = (1 as u64) << index;

        self.mailbox[index] = Square {
            piece: PieceType::None,
            color: Color::None,
        };

        if color == Color::White {
            self.white_bb &= !bb;
        } else {
            self.black_bb &= !bb;
        }

        match piece {
            PieceType::Pawn => {
                self.pawn_bbs[color as usize] &= !bb;
            }
            PieceType::Knight => {
                self.knight_bbs[color as usize] &= !bb;
            }
            PieceType::Bishop => {
                self.bishop_bbs[color as usize] &= !bb;
            }
            PieceType::Rook => {
                self.rook_bbs[color as usize] &= !bb;
            }
            PieceType::Queen => {
                self.queen_bbs[color as usize] &= !bb;
            }
            PieceType::King => {
                self.king_bbs[color as usize] &= !bb;
            }
            _ => {}
        }

        (color, piece)
    }

    pub fn move_piece(&mut self, from: usize, to: usize) {
        let (color, piece) = self.remove_piece(from);
        self.remove_piece(to);
        self.add_piece(color, piece, to);
    }

    pub fn load_fen(&mut self, fen: &str) {
        let split_fen: Vec<&str> = fen.split(' ').collect();
        let position: &str = split_fen[0];
        self.wtm = split_fen[1] == "w";

        self.mailbox = [Square {
            piece: PieceType::None,
            color: Color::None,
        }; 64];

        self.pawn_bbs = [0; 2];
        self.knight_bbs = [0; 2];
        self.bishop_bbs = [0; 2];
        self.rook_bbs = [0; 2];
        self.queen_bbs = [0; 2];
        self.king_bbs = [0; 2];

        self.white_bb = 0;
        self.black_bb = 0;

        self.reversables = 0;
        self.halfmove_clock = 0;
        self.castling_rights = 0b1111; // KQkq

        let (mut rank, mut file) = (7, 0);
        for c in position.chars() {
            if c == '/' {
                rank -= 1;
                file = 0
            } else if c.is_numeric() {
                file += c.to_digit(10).unwrap();
            } else {
                let is_white = c.is_uppercase();

                let mailbox_idx = (rank * 8 + file) as usize;
                let index: Bitboard = (1 as Bitboard) << mailbox_idx;

                if is_white {
                    self.white_bb |= index;
                } else {
                    self.black_bb |= index;
                }

                let color = if is_white { Color::White } else { Color::Black };
                self.mailbox[mailbox_idx].color = color;

                match c.to_lowercase().next().unwrap() {
                    'p' => {
                        self.add_piece(color, PieceType::Pawn, mailbox_idx);
                    }
                    'n' => {
                        self.add_piece(color, PieceType::Knight, mailbox_idx);
                    }
                    'b' => {
                        self.add_piece(color, PieceType::Bishop, mailbox_idx);
                    }
                    'r' => {
                        self.add_piece(color, PieceType::Rook, mailbox_idx);
                    }
                    'q' => {
                        self.add_piece(color, PieceType::Queen, mailbox_idx);
                    }
                    'k' => {
                        self.add_piece(color, PieceType::King, mailbox_idx);
                    }
                    // ...
                    // lots of tedious code rewriting here
                    _ => {
                        panic!("bad fen")
                    }
                }
                file += 1;
            }
        }
    }

    pub fn get_occupied_squares(&self) -> Bitboard {
        self.white_bb | self.black_bb
    }

    pub fn get_knight_targets(&self, color: Color) -> Bitboard {
        let mut knights = self.knight_bbs[color as usize];
        let us = [self.white_bb, self.black_bb][color as usize];
        let mut from = knights.pop_lsb();

        let mut bb: Bitboard = 0;

        while from != 65 {
            let mut not_blocked = HORSEY_AVECS[from as usize] & !us;

            let mut index = not_blocked.pop_lsb();
            while index != 65 {
                let to = index;
                bb |= (1 as u64) << to;
                index = not_blocked.pop_lsb();
            }

            from = knights.pop_lsb();
        }

        bb
    }

    pub fn get_king_targets(&self, color: Color) -> Bitboard {
        let mut kings = self.king_bbs[color as usize];
        let us = [self.white_bb, self.black_bb][color as usize];

        let mut bb: Bitboard = 0;

        let from = kings.pop_lsb();

        let mut not_blocked = CROWNIES_AVECS[from] & !us;

        let mut index = not_blocked.pop_lsb();
        while index != 65 {
            bb |= (1 as u64) << index;
            index = not_blocked.pop_lsb();
        }

        bb
    }

    fn get_bishop_attacks(&self, index: usize) -> Bitboard {
        let mut bb: Bitboard = 0;

        let avec = BISHOP_AVECS[index];

        let blockers = avec & self.get_occupied_squares();

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

        bb
    }

    pub fn get_bishop_targets(&self, color: Color) -> Bitboard {
        let us = [self.white_bb, self.black_bb][color as usize];
        let mut bishops = self.bishop_bbs[color as usize];

        let mut bb: Bitboard = 0;

        let mut from = bishops.pop_lsb();
        while from != 65 {
            let mut moves_bb = self.get_bishop_attacks(from) & !us;
            let mut to = moves_bb.pop_lsb();
            while to != 65 {
                bb |= (1 as u64) << to;
                to = moves_bb.pop_lsb();
            }
            from = bishops.pop_lsb();
        }

        bb
    }

    fn get_rook_attacks(&self, index: usize) -> Bitboard {
        let mut bb: Bitboard = 0;

        let avec = ROOK_AVECS[index];

        let blockers = avec & self.get_occupied_squares();

        let (file, rank) = idx_to_file_rank(index as u8);

        // South
        for i in 1..rank + 1 {
            let r = rank - i;
            let sq = (1 as u64) << idx_from_file_rank(file, r);
            bb |= sq;
            if sq & blockers > 0 {
                break;
            }
        }
        // West
        for i in 1..file + 1 {
            let f = file - i;
            let sq = (1 as u64) << idx_from_file_rank(f, rank);
            bb |= sq;
            if sq & blockers > 0 {
                break;
            }
        }
        // North
        for i in 1..8 - rank {
            let r = rank + i;
            let sq = (1 as u64) << idx_from_file_rank(file, r);
            bb |= sq;
            if sq & blockers > 0 {
                break;
            }
        }
        // East
        for i in 1..8 - file {
            let f = file + i;
            let sq = (1 as u64) << idx_from_file_rank(f, rank);
            bb |= sq;
            if sq & blockers > 0 {
                break;
            }
        }

        bb
    }

    pub fn get_rook_targets(&self, color: Color) -> Bitboard {
        let us = [self.white_bb, self.black_bb][color as usize];
        let mut rooks = self.rook_bbs[color as usize];

        let mut bb: Bitboard = 0;

        let mut from = rooks.pop_lsb();
        while from != 65 {
            let mut moves_bb = self.get_rook_attacks(from) & !us;
            let mut to = moves_bb.pop_lsb();
            while to != 65 {
                bb |= (1 as u64) << to;
                to = moves_bb.pop_lsb();
            }
            from = rooks.pop_lsb();
        }
        bb
    }

    pub fn get_queen_targets(&self, color: Color) -> Bitboard {
        self.get_bishop_targets(color) | self.get_rook_targets(color)
    }

    pub fn get_pawn_targets(&self, color: Color) -> Bitboard {
        let mut pawns = self.pawn_bbs[color as usize];
        let us = [self.white_bb, self.black_bb][color as usize];
        let opp = [self.black_bb, self.white_bb][color as usize];
        let mut from = pawns.pop_lsb();

        let mut bb = 0;

        while from != 65 {
            let not_blocked = PAWN_AVECS[from as usize] & !us;
            let (file, _rank) = idx_to_file_rank(from as u8);
            let not_capture = not_blocked & gen_file_mask(file);
            let captures = (not_blocked & !not_capture) & opp;
            let mut valids = not_capture | captures;

            let mut index = valids.pop_lsb();
            while index != 65 {
                let to = index;
                bb |= (1 as u64) << to;
                index = valids.pop_lsb();
            }

            from = pawns.pop_lsb();
        }

        bb
    }

    pub fn get_all_targets(&self, color: Color) -> Bitboard {
        self.get_knight_targets(color)
            | self.get_king_targets(color)
            | self.get_bishop_targets(color)
            | self.get_rook_targets(color)
            | self.get_queen_targets(color)
            | self.get_pawn_targets(color)
    }
}

pub fn pretty_print_index(index: u8) -> String {
    let (file, rank) = idx_to_file_rank(index);
    let f = "abcdefgh".as_bytes()[file as usize] as char;
    let r = char::from_digit(rank as u32 + 1, 10).unwrap();
    return f.to_string() + &r.to_string();
}
