use bitboard::Bitboard;

struct Magic {
    mask:    Bitboard,
    magic:   Bitboard,
    attacks: Bitboard*,  // TODO pointer...?
    shift: u64,
}

impl Magic {
    // occupied == bb_all
    pub fn get_attacks(&self, occupied: Bitboard) -> Bitboard {
        m.attacks[m.get_index(occupied)]
    }

    fn get_index(&self, occupied: Bitboard) -> usize {
        (((occupied.0 & self.mask.0) * self.magic.0) >> self.shift) as usize
    }
}

// Probably put this inside movegen...
let static mut magics [Magic; 64];
let static mut attacks [Bitboard; ...];




// TODO
// init_attacks
// init_magics


