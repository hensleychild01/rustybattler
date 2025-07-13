#[derive(Clone, Copy)]
pub enum PieceType {
    None,
    Pawn, 
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone)]
pub enum Color {
    None,
    White,
    Black
}