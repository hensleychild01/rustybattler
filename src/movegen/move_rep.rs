use crate::{board::Board, enums::{Color, PieceType}};

pub struct UnpackedMove {
    from: u8, 
    to: u8,

    color: Color,

    is_capture: bool,
    is_castle: bool,

    captured: PieceType,
    promo_to: PieceType
}

pub type Move = u32;

pub type MoveList = Vec<Move>;

pub trait MoveListExt {
    fn gen_sliding_moves(&mut self, b: &Board) {}
    fn gen_knight_moves(&mut self, b: &Board) {}
}

impl MoveListExt for MoveList {
    fn gen_sliding_moves(&mut self, b: &Board) {}
    fn gen_knight_moves(&mut self, b: &Board) {
        let deref_board = *b;

        let color = if deref_board.wtm {0} else {1};
        let our_knights = deref_board.knight_bbs[color];
    }
}

impl Board {
    pub fn make_move(&mut self, m: Move) {}
    pub fn unmake_move(&mut self, m: Move) {}
}

// discord