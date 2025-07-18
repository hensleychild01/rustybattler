use crate::{board::Board, movegen::{move_rep::{MoveList, MoveListPrettyPrint}, pseudolegals::MoveListExt}};

mod bitboards;
mod board;
mod enums;
mod movegen;



fn main() {
    let mut game = Board::get_empty_board();
    game.init();

    let mut moves: MoveList = vec![];
    moves.gen_king_moves(&game, enums::Color::Black);

    moves.print();
}
