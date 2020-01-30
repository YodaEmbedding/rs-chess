use std::fmt;
use std::option::Option;
use std::string::String;

use arrayvec::ArrayVec;

use crate::moves::Move;

// enum Direction {
//   North =  8,
//   East  =  1,
//   South = -8,
//   West  = -1,
//   NorthEast = North + East,
//   SouthEast = South + East,
//   SouthWest = South + West,
//   NorthWest = North + West
// };

pub const COLOR_SIZE: usize = 2;
pub const PIECE_NAME_SIZE: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    //All,
    //Empty,
}

// Consider making this a tuple struct?
// Or a uchar for space savings; with accessor methods for extracting pieceName/color
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_name: PieceName,
    pub color: Color,
}

#[derive(Debug)]
pub struct PieceBoard(pub ArrayVec<[Option<Piece>; 64]>);

impl Color {
    pub fn opposite(&self) -> Self {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn to_int(&self) -> i32 {
        match *self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

impl Piece {
    pub fn new(piece_name: PieceName, color: Color) -> Self {
        Self {
            piece_name: piece_name,
            color: color,
        }
    }
}

impl PieceBoard {
    pub fn new_default() -> Self {
        use Color::*;
        use PieceName::*;

        let xx = None::<Piece>;
        let wp = Some(Piece::new(Pawn, White));
        let wn = Some(Piece::new(Knight, White));
        let wb = Some(Piece::new(Bishop, White));
        let wr = Some(Piece::new(Rook, White));
        let wq = Some(Piece::new(Queen, White));
        let wk = Some(Piece::new(King, White));
        let bp = Some(Piece::new(Pawn, Black));
        let bn = Some(Piece::new(Knight, Black));
        let bb = Some(Piece::new(Bishop, Black));
        let br = Some(Piece::new(Rook, Black));
        let bq = Some(Piece::new(Queen, Black));
        let bk = Some(Piece::new(King, Black));

        // TODO incomplete table
        #[rustfmt::skip]
        let piece_board = PieceBoard(ArrayVec::from([
            wr, wn, wb, wq, wk, wb, wn, wr,
            wp, wp, wp, wp, wp, wp, wp, wp,
            xx, xx, xx, xx, xx, xx, xx, xx,
            xx, xx, xx, xx, xx, xx, xx, xx,
            xx, xx, xx, xx, xx, xx, xx, xx,
            xx, xx, xx, xx, xx, xx, xx, xx,
            bp, bp, bp, bp, bp, bp, bp, bp,
            br, bn, bb, bq, bk, bb, bn, br,
        ]));

        piece_board
    }

    pub fn make_move(&self, move_: Move) -> Self {
        let from = move_.get_from();
        let to = move_.get_to();

        let mut pb = self.clone();
        pb.0[to.0 as usize] = pb.0[from.0 as usize];
        pb.0[from.0 as usize] = None;
        pb
    }
}

impl PieceName {
    fn to_char(&self) -> char {
        match *self {
            PieceName::Pawn => 'p',
            PieceName::Knight => 'n',
            PieceName::Bishop => 'b',
            PieceName::Rook => 'r',
            PieceName::Queen => 'q',
            PieceName::King => 'k',
        }
    }
}

impl Clone for PieceBoard {
    fn clone(&self) -> Self {
        PieceBoard(self.0.clone())
    }
}

impl fmt::Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NOTE: the white and black pieces are inverted,
        // since they appear differently on an inverted terminal
        let white = ["♟", "♞", "♝", "♜", "♛", "♚"];
        let black = ["♙", "♘", "♗", "♖", "♕", "♔"];

        write!(
            f,
            "{}",
            match self.color {
                Color::White => white[self.piece_name as usize],
                Color::Black => black[self.piece_name as usize],
            }
        )
    }
}

impl fmt::Display for PieceBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn opt_piece_to_str(piece: &Option<Piece>) -> String {
            match *piece {
                Some(ref p) => format!("{}", p),
                None => String::from("."),
            }
        }

        let xs = self.0.iter().map(opt_piece_to_str).collect::<Vec<_>>();
        let rows = xs.chunks(8).map(|x| x.join(" ")).rev().collect::<Vec<_>>();
        let grid = rows.join("\n");

        write!(f, "{}", grid)
    }
}
