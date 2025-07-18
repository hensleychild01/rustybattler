use crate::{
    board::{Board, pretty_print_index},
    enums::{Color, PieceType},
};

pub struct UnpackedMove {
    from: u8,
    to: u8,

    color: Color,

    is_capture: bool,
    is_castle: bool,

    captured: PieceType,
    promo_to: PieceType,
}

pub type Move = u32;
// rightmost 6 bits are from-square, then 6 for to-square
// then 3 bits for from-piece (mover), then 3 for to-piece (captured)
//

pub type MoveList = Vec<Move>;

pub trait MoveListPrettyPrint {
    fn print(&self);
}

impl MoveListPrettyPrint for MoveList {
    fn print(&self) {
        for m in self {
            let from = pretty_print_index((m & 0b111111) as u8);
            let to = pretty_print_index(((m & 0b111111000000) >> 6) as u8);
            println!("{} to {}", from, to);
        }
    }
}
