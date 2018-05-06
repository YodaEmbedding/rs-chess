use bitboard::Bitboard;
use position::Position;

extern crate arrayvec;
use self::arrayvec::ArrayVec;

pub type Square = u32;
pub struct Move(pub u32);

// TODO why are some of these needed?
enum MoveFlags {
    CastleKing,
    CastleQueen,
    Capture,
    DoublePawn,
    Enpassant,
}

impl Move {
    pub fn new(from: Square, to: Square, flags: u32) -> Self {
        Move(from | to << 6 | flags << 12)
    }
}

// static mut RookAttackMap: ArrayVec<[Bitboard; 64]> = ArrayVec::from([Bitboard(0); 64]);
static mut RookAttackMap: [Bitboard; 64] = [Bitboard(0); 64];
// static const   PawnAttackMap: ArrayVec<[Bitboard; 64]>;
// static const BishopAttackMap: ArrayVec<[Bitboard; 64]>;

impl Position {
    // pub fn find_moves(&self) -> [Moves] {
    //     
    // }
    //
    // fn get_pawn_attacks(&self) -> [Moves] {
    //
    // }

}

// pub fn init_attacks() {
//     init_rook_attacks();
// }

pub fn init_rook_attacks() -> ArrayVec<[Bitboard; 64]> {
    let rank_mask = 0x00000000000000FFu64;
    let file_mask = 0x0101010101010101u64;

    let mut rook_map: ArrayVec<[Bitboard; 64]> = ArrayVec::new();

    // TODO enumerate?
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        rook_map.push(Bitboard(rank_mask << 8*rank | file_mask << file));
    }

    rook_map
}

// Magics?

