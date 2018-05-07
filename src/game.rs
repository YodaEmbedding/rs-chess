use bitboard;
use bitboard::Bitboard;

use arrayvec::ArrayVec;

type AttackMap = ArrayVec<[Bitboard; 64]>;

pub struct Game {
    // RookAttackMap: ArrayVec<[Bitboard; 64]> = ArrayVec::from([Bitboard(0); 64]);
    // RookAttackMap: [Bitboard; 64] = [Bitboard(0); 64];
    // TODO temporarily public
    // TODO consider moving these into a MoveGenerator class?
    pub pawn_attack_map:   AttackMap,
    pub bishop_attack_map: AttackMap,
    pub rook_attack_map:   AttackMap,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pawn_attack_map:    Self::get_pawn_attacks(),
            bishop_attack_map:  Self::get_bishop_attacks(),
            rook_attack_map:    Self::get_rook_attacks(),
        }
    }

    fn get_pawn_attacks() -> AttackMap {
        let mut attack_map = AttackMap::new();

        for i in 0..64 {
            let sq = Bitboard(1 << i);
            let bb = Bitboard(
                sq.shift_up().shift_left().0 |
                sq.shift_up().shift_right().0);
            attack_map.push(bb);
        }

        attack_map
    }

    fn get_bishop_attacks() -> AttackMap {
        let mut attack_map = AttackMap::new();

        for i in 0..64 {
            let rank = i / 8;
            let file = i % 8;
            let bb = Bitboard(
                bitboard::A1H8.shift_up_n  (  rank).shift_right_n(  file).0 |
                bitboard::A1H8.shift_down_n(7-rank).shift_left_n (7-file).0 |
                bitboard::A8H1.shift_up_n  (  rank).shift_left_n (7-file).0 |
                bitboard::A8H1.shift_down_n(7-rank).shift_right_n(  file).0);
            attack_map.push(bb);
        }

        attack_map
    }

    fn get_rook_attacks() -> AttackMap {
        let mut attack_map = AttackMap::new();

        for i in 0..64 {
            let rank = i / 8;
            let file = i % 8;
            let bb = Bitboard(
                bitboard::Rank1.shift_up_n(rank).0 |
                bitboard::FileA.shift_right_n(file).0);
            attack_map.push(bb);
        }

        attack_map
    }
}

