use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

impl Bitboard {
    // fn func(&self) -> Bitboard { self.0 }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bin = format!("{:064b}", self.0);

        let xs = bin.chars()
            .map(|x| if x == '0' {'.'} else {x})
            .rev()
            .collect::<Vec<_>>();

        let rows = xs.chunks(8)
            .map(|x| x.into_iter().collect::<String>())
            .rev()
            .collect::<Vec<_>>();

        let grid = rows.join("\n");
        write!(f, "{}", grid)
    }
}

