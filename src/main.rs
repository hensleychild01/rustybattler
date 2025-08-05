use crate::{bitboards::BitboardExt, board::Board, enums::Color};

mod bitboards;
mod board;
mod enums;
mod movegen;

fn main() {
    let mut game = Board::get_empty_board();
    game.init();
    game.remove_piece(25);
    game.white_bb.print();
}
