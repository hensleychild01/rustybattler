use crate::bitboards::Bitboard;

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
