use crate::enums::{Color, PieceType};
use crate::Bitboard;

#[derive(Copy, Clone)]
pub struct Square {
    pub piece: PieceType,
    pub color: Color,
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
            mailbox: [Square{piece: PieceType::None, color:Color::None}; 64],
            white_bb: 0,
            black_bb: 0,
        };
}

    pub fn init(&mut self) {
        self.load_fen(STARTING_FEN);
    }

    pub fn load_fen(&mut self, fen: &str) {
        let split_fen: Vec<&str> = fen.split(' ').collect();
        let position: &str = split_fen[0];
        self.wtm = split_fen[1] == "w";

        self.mailbox = [Square {
            piece: PieceType::None,
            color: Color::None,
        }; 64];

        // white first
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
                let color_idx = if is_white { 0 } else { 1 };

                let mailbox_idx = (rank * 8 + file) as usize;
                let index: Bitboard = (1 as Bitboard) << mailbox_idx;

                if is_white {
                    self.white_bb |= index;
                } else {
                    self.black_bb |= index;
                }

                self.mailbox[mailbox_idx].color =
                    if is_white { Color::White } else { Color::Black };

                match c.to_lowercase().next().unwrap() {
                    'p' => {
                        self.pawn_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::Pawn;
                    }
                    'n' => {
                        self.knight_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::Knight;
                    }
                    'b' => {
                        self.bishop_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::Bishop;
                    }
                    'r' => {
                        self.rook_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::Rook;
                    }
                    'q' => {
                        self.queen_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::Queen;
                    }
                    'k' => {
                        self.king_bbs[color_idx] |= index;
                        self.mailbox[mailbox_idx].piece = PieceType::King;
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
}
