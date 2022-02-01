use std::ops::BitOr;
use std::ops::BitAnd;
use std::fmt;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct Board {
    spaces: [[Piece; 8]; 8],
}

impl Board {
    pub fn default() -> Board {
        let mut spaces: [[Piece; 8]; 8] = [[Piece::Empty; 8]; 8];

        spaces[0][0] = Piece::WhiteRook;
        spaces[0][1] = Piece::WhiteKnight;
        spaces[0][2] = Piece::WhiteBishop;
        spaces[0][3] = Piece::WhiteQueen;
        spaces[0][4] = Piece::WhiteKing;
        spaces[0][5] = Piece::WhiteBishop;
        spaces[0][6] = Piece::WhiteKnight;
        spaces[0][7] = Piece::WhiteRook;

        spaces[7][0] = Piece::BlackRook;
        spaces[7][1] = Piece::BlackKnight;
        spaces[7][2] = Piece::BlackBishop;
        spaces[7][3] = Piece::BlackQueen;
        spaces[7][4] = Piece::BlackKing;
        spaces[7][5] = Piece::BlackBishop;
        spaces[7][6] = Piece::BlackKnight;
        spaces[7][7] = Piece::BlackRook;

        for i in 0..8 {
            spaces[1][i] = Piece::WhitePawn;
            spaces[6][i] = Piece::BlackPawn;
        }

        Board {
            spaces: spaces,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = 33;
        let mut output: String = "".to_owned();

        for _i in 0..width {
            output.push_str("—");
        }
        output.push_str("\n");

        for row in self.spaces.iter().rev() {
            output.push_str("│");
            for piece in row {
                output.push_str(&format!(" {} │", piece));
            }
            output.push_str("\n");
            for _i in 0..width {
                output.push_str("—");
            }
            output.push_str("\n");
        }

        write!(f, "{}", output)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Copy, Clone, FromPrimitive)]
pub enum Piece {
    // last three bits represent type
    Empty = 0b00000,

    Pawn    = 0b00001,
    Knight  = 0b00010,
    Bishop  = 0b00011,
    Rook    = 0b00100,
    Queen   = 0b00101,
    King    = 0b00110,

    // first two bits represent color
    Black   = 0b01000,
    White   = 0b10000,

    BlackPawn = Piece::Pawn as isize | Piece::Black as isize,
    BlackKnight = Piece::Knight as isize | Piece::Black as isize,
    BlackBishop = Piece::Bishop as isize | Piece::Black as isize,
    BlackRook = Piece::Rook as isize | Piece::Black as isize,
    BlackQueen = Piece::Queen as isize | Piece::Black as isize,
    BlackKing = Piece::King as isize | Piece::Black as isize,

    WhitePawn = Piece::Pawn as isize | Piece::White as isize,
    WhiteKnight = Piece::Knight as isize | Piece::White as isize,
    WhiteBishop = Piece::Bishop as isize | Piece::White as isize,
    WhiteRook = Piece::Rook as isize | Piece::White as isize,
    WhiteQueen = Piece::Queen as isize | Piece::White as isize,
    WhiteKing = Piece::King as isize | Piece::White as isize,
}

impl BitAnd for Piece {
    type Output = Self;
    // rhs is the "right-hand side" of the expression `a | b`
    fn bitand(self, rhs: Self) -> Self::Output {
        Piece::from_isize(self as isize & rhs as isize)
    }
}

impl BitOr for Piece {
    type Output = Self;
    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self::Output {
        Piece::from_isize(self as isize | rhs as isize)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (Piece::color(*self), Piece::r#type(*self)) {
            (_, Piece::Empty) => write!(f, " "),

            (Piece::Black, Piece::Pawn) => write!(f, "p"),
            (Piece::Black, Piece::Knight) => write!(f, "n"),
            (Piece::Black, Piece::Bishop) => write!(f, "b"),
            (Piece::Black, Piece::Rook) => write!(f, "r"),
            (Piece::Black, Piece::Queen) => write!(f, "q"),
            (Piece::Black, Piece::King) => write!(f, "k"),

            (Piece::White, Piece::Pawn) => write!(f, "P"),
            (Piece::White, Piece::Knight) => write!(f, "N"),
            (Piece::White, Piece::Bishop) => write!(f, "B"),
            (Piece::White, Piece::Rook) => write!(f, "R"),
            (Piece::White, Piece::Queen) => write!(f, "Q"),
            (Piece::White, Piece::King) => write!(f, "K"),

            _ => write!(f, "?")
        }
    }
}

impl Piece {
    fn from_isize(piece: isize) -> Piece {
        FromPrimitive::from_isize(piece).unwrap()
    }

    fn r#type(piece: Piece) -> Piece {
        Piece::from_isize(piece as isize & 0b00111)
    }

    fn color(piece: Piece) -> Piece {
        Piece::from_isize(piece as isize & 0b11000)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------