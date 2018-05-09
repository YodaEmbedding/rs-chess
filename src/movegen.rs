use arrayvec::ArrayVec;

use bitboard;
use bitboard::Bitboard;
use position::Position;
use pieces::{Color, PieceName};
use moves::Move;

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
        // TODO maybe accept these as arguments instead of recomputing
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
        let mut moves: Vec<Move> = Vec::with_capacity(Self::INITIAL_MOVELIST_CAPACITY);

        // TODO position.turn.opposite()
        let player_color = Color::White;
        let enemy_color  = Color::Black;

        let pieces = position.piece_board.0.iter()
            .filter(|op| match op {
                Some(p) => p.color == player_color,
                None => false
            });

        for (i, piece) in pieces.enumerate() {
            // switch by piece type (or consider no-cost polymorphism?)
            // for pawns, loop through all squares of attack map (check legality only for if piece exists) and also legal single pawn pushes

            // TODO move pattern match into separate function
            // let move_squares = match piece.piece_name {
            //     PieceName::Pawn => get_pawn_attacks(),
            //     _               => get_pawn_attacks() // TODO
            // };

            // moves.append(&mut ummmm you should probably construct the other vector... or push into this one directly over iteration?)
            // moves.extend()
        }

        // get_valid_attacks() // different for sliding?
        // if bish,queen,rook, do get_moves_sliding()

        moves
    }

    // fn get_pawn_attacks(&self) -> Iterator<> {
    //     pawn_attack_map
    // }
}

impl Position {
    pub fn get_moves(&self, move_generator: MoveGenerator) -> Vec<Move> {
        move_generator.get_moves(self)
    }
}

// Magics?

