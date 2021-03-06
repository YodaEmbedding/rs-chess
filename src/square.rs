use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Square(pub u32);

impl Square {
    #[inline]
    pub fn get_file(&self) -> u8 {
        (self.0 & 0x07) as u8
    }

    #[inline]
    pub fn get_rank(&self) -> u8 {
        (self.0 >> 3 & 0x07) as u8
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.get_file() as u8 + 'a' as u8) as char,
            (self.get_rank() as u8 + '1' as u8) as char
        )
    }
}
