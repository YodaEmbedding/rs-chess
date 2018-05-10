use arrayvec::ArrayVec;

use bitboard;
use bitboard::{Bitboard, BitboardIterator};
use moves::Move;
use pieces::{Color, PieceName};
use position::Position;
use square::Square;

type AttackMap = ArrayVec<[Bitboard; 64]>;

pub struct MoveGenerator {
    // TODO why are these public?
    pub pawn_attack_map:   AttackMap,
    pub bishop_attack_map: AttackMap,
    pub rook_attack_map:   AttackMap,
    pub queen_attack_map:  AttackMap,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            pawn_attack_map:   Self::make_pawn_attack_map(),
            bishop_attack_map: Self::make_bishop_attack_map(),
            rook_attack_map:   Self::make_rook_attack_map(),
            queen_attack_map:  Self::make_queen_attack_map(),
        }
    }

    // TODO consider moving these outside this class...
    // maybe a AttackMapGenerator?
    fn make_pawn_attack_map() -> AttackMap {
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

    fn make_bishop_attack_map() -> AttackMap {
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

    fn make_rook_attack_map() -> AttackMap {
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

    fn make_queen_attack_map() -> AttackMap {
        let mut attack_map = AttackMap::new();
        let rook_attack_map   = Self::make_rook_attack_map();
        let bishop_attack_map = Self::make_bishop_attack_map();

        for i in 0..64 {
            let bb = Bitboard(rook_attack_map[i].0 | bishop_attack_map[i].0);
            attack_map.push(bb);
        }

        attack_map
    }
}

impl MoveGenerator {
    const INITIAL_MOVELIST_CAPACITY: usize = 64;

    pub fn get_moves(&self, position: &Position) -> Vec<Move> {
        let ally_color  = position.turn;
        let enemy_color = position.turn.opposite();

        let pieces = position.piece_board.0.iter()
            .enumerate()
            .filter(|(i, p)| p.is_some())
            .map(   |(i, p)| (i, p.unwrap()))
            .filter(|(i, p)| p.color == ally_color);

        let mut movelist: Vec<Vec<Move>> = Vec::with_capacity(Self::INITIAL_MOVELIST_CAPACITY);

        for (i, piece) in pieces {
            let from = Square(i as u32);

            let move_squares = match piece.piece_name {
                PieceName::Pawn   => Self::get_pawn_attacks(   position, from, &self.pawn_attack_map),
                PieceName::Bishop => Self::get_sliding_attacks(position, from, &self.bishop_attack_map),
                PieceName::Rook   => Self::get_sliding_attacks(position, from, &self.rook_attack_map),
                PieceName::Queen  => Self::get_sliding_attacks(position, from, &self.queen_attack_map),
                _                 => bitboard::Empty.iter()
            };

            // TODO flags...?
            let pseudo_moves = move_squares
                .map(|to| Move::new(from, to, 0x00))
                .collect::<Vec<Move>>();

            movelist.push(pseudo_moves);
        }

        // get_valid_attacks() // different for sliding?
        // if bish,queen,rook, do get_moves_sliding()

        movelist.into_iter().flat_map(|x| x).collect::<Vec<_>>()
    }

    fn get_pawn_attacks(position: &Position, square: Square,
        attack_map: &AttackMap) -> BitboardIterator {

        fn forward(turn: Color, bitboard: Bitboard, n: u64) -> Bitboard {
            match turn {
                Color::White => Bitboard(bitboard.0 << n),
                Color::Black => Bitboard(bitboard.0 >> n)
            }
        }

        let turn = position.turn;
        let sq_bb = Bitboard::from(square);
        let move1 = forward(turn, sq_bb, 8);
        let move2 = forward(turn, sq_bb, 16);

        let unoccupied = Bitboard(!position.get_bb_all().0);
        let unoccupied_prev = forward(turn, unoccupied, 8);
        let single_move = Bitboard(move1.0 & unoccupied.0);
        let double_move = Bitboard(move2.0 & unoccupied.0 & unoccupied_prev.0);

        let attacks = attack_map[square.0 as usize];
        let captures = Bitboard(attacks.0 & position.get_bb_enemy().0);

        Bitboard(captures.0 | single_move.0 | double_move.0).iter()
    }

    fn get_sliding_attacks(position: &Position, square: Square,
        attack_map: &AttackMap) -> BitboardIterator {

        let turn = position.turn;
        let sq_bb = Bitboard::from(square);

        let attacks = attack_map[square.0 as usize];
        let captures = Bitboard(attacks.0 & position.get_bb_enemy().0);

        // TODO use magics?
        // How do we fill only relevant attacks?
        // Also, should the attack_maps mask away:
        //   - own piece square (!!!)
        //   - redundant squares

        bitboard::Empty.iter()
    }
}

// Magics?

