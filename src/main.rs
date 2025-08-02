use crate::{
    bitboards::{Bitboard, BitboardExt}, board::Board, movegen::{
        move_rep::{MoveList, MoveListPrettyPrint},
        pseudolegals::{get_bishop_attacks, MoveListExt},
    }
};

mod bitboards;
mod board;
mod enums;
mod movegen;

fn main() {
    let mut game = Board::get_empty_board();
    game.load_fen("2b5/p2NBp1p/1bp1nPPr/3P4/2pRnr1P/1k1B1Ppp/1P1P1pQP/Rq1N3K w - - 0 1");

    game.get_moves().print();
}
