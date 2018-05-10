use std::fmt;

use arrayvec::ArrayVec;

use bitboard::Bitboard;
use movegen::MoveGenerator;
use moves::Move;
use pieces::*;

#[derive(Debug)]
pub struct Position {
    // Vec<Bitboard> bitboards,
    // GameState game_state,

    pub bitboard_piece: ArrayVec<[Bitboard; PIECE_NAME_SIZE]>,
    pub bitboard_color: ArrayVec<[Bitboard; COLOR_SIZE]>,
    pub piece_board: PieceBoard,
    pub turn: Color,
    // pub ply: usize,
    // pub hash: Hash,
    // pub castling: u8,
    // pub enpassant: u64,
    // Bitboard bitboard_all,

    // int pieceCount[PieceName::MAX * Color::MAX],
    // Square pieceList[PieceName::MAX * Color::MAX][16],
    // int index[64],
    // int castlingRightsMask[64],
    // Square castlingRookSquare[CASTLING_RIGHT_NB],
    // Bitboard castlingPath[CASTLING_RIGHT_NB],
    // int gamePly,
    // Color sideToMove,
    // Thread* thisThread,
    // StateInfo* st,
}

impl Position {
    pub fn new_default() -> Self {
        let bitboard_piece = ArrayVec::from([
            Bitboard(0x00FF00000000FF00u64),  // Pawn
            Bitboard(0x4200000000000042u64),  // Bishop
            Bitboard(0x2400000000000024u64),  // Knight
            Bitboard(0x8100000000000081u64),  // Rook
            Bitboard(0x0800000000000008u64),  // Queen
            Bitboard(0x1000000000000010u64),  // King
        ]);

        let bitboard_color = ArrayVec::from([
            Bitboard(0x000000000000FFFFu64),  // White
            Bitboard(0xFFFF000000000000u64),  // Black
        ]);

        Self {
            bitboard_piece: bitboard_piece,
            bitboard_color: bitboard_color,
            piece_board: PieceBoard::new_default(),
            turn: Color::White,
        }
    }

    pub fn get_moves(&self, move_generator: &MoveGenerator) -> Vec<Move> {
        move_generator.get_moves(self)
    }
}

impl fmt::Display for Position {
    // extract to separate display functions for different things...?
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} to move\n{}", self.turn, self.piece_board)
    }
}

