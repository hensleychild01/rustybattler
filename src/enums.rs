#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    None = 2,
    White = 0,
    Black = 1,
}
