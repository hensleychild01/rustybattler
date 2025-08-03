pub type Bitboard = u64;

pub trait BitboardExt {
    fn print(&self);
    fn count(&self) -> i32;
    fn pop_lsb(&mut self) -> usize;
}

impl BitboardExt for Bitboard {
    fn print(&self) {
        let mut s: String = "\n".to_string();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = (rank * 8 + file) as u64;
                let bit = (1 as u64) << index;

                if file == 0 {
                    s += &(char::from_digit(rank + 1, 10).unwrap().to_string());
                }

                if (*self & bit) > 0 {
                    s += " # ";
                } else {
                    s += " . ";
                }
            }

            s += "\n";
        }

        s += "  a  b  c  d  e  f  g  h";
        println!("{s}");
    }
    fn count(&self) -> i32 {
        let mut count = 0;
        for i in 0..64 {
            if (*self & (1 << i)) > 0 {
                count += 1;
            }
        }
        count
    }
    fn pop_lsb(&mut self) -> usize {
        let trailers = self.trailing_zeros() as usize;
        if trailers == 64 {return 65;}
        *self ^= (1 as u64) << trailers;
        trailers
    }
}