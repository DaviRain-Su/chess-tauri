use std::fmt;

// row number
pub const ROW_NUM: usize = 9;
// column number
pub const COLUMN_NUM: usize = 10;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    King,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match (self.color, self.piece_type) {
            (Color::Red, PieceType::King) => "帥",
            (Color::Red, PieceType::Advisor) => "仕",
            (Color::Red, PieceType::Elephant) => "相",
            (Color::Red, PieceType::Horse) => "傌",
            (Color::Red, PieceType::Chariot) => "俥",
            (Color::Red, PieceType::Cannon) => "炮",
            (Color::Red, PieceType::Soldier) => "兵",
            (Color::Black, PieceType::King) => "將",
            (Color::Black, PieceType::Advisor) => "士",
            (Color::Black, PieceType::Elephant) => "象",
            (Color::Black, PieceType::Horse) => "馬",
            (Color::Black, PieceType::Chariot) => "車",
            (Color::Black, PieceType::Cannon) => "砲",
            (Color::Black, PieceType::Soldier) => "卒",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WrapPieces {
    pub id: usize,
    pub point: Point,
    pub priece: Option<Piece>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Self { x, y }
    }
}

impl WrapPieces {
    pub fn new() -> Self {
        Self {
            id: 0,
            point: Point::default(),
            priece: None,
        }
    }
}

fn cal_idx(x: usize, y: usize) -> usize {
    x * ROW_NUM + y
}

pub type Pieces = [[WrapPieces; ROW_NUM]; COLUMN_NUM];

pub struct ChessBoard {
    pieces: Pieces,
}

impl ChessBoard {
    /// 棋盘的初始化
    pub fn new() -> Self {
        let mut pieces = [[WrapPieces::new(); ROW_NUM]; COLUMN_NUM];

        /// 先默认初始化所有的棋盘
        for i in 0..COLUMN_NUM {
            for j in 0..ROW_NUM {
                pieces[i][j] = WrapPieces {
                    id: i * ROW_NUM + j,
                    point: Point::new(i, j),
                    priece: None,
                };
            }
        }

        for i in 0..ROW_NUM {
            // 初始化黑棋的士兵
            pieces[3][i] = WrapPieces {
                id: cal_idx(3, i),
                point: Point::new(3, i),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Soldier,
                }),
            };
            // 初始化红旗的士兵
            pieces[6][i] = WrapPieces {
                id: cal_idx(6, i),
                point: Point::new(6, i),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Soldier,
                }),
            };
            
            // 初始化黑棋的炮
            pieces[2][1] = WrapPieces {
                id: cal_idx(2,1),
                point: Point::new(2,1),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Cannon,
                }),
            };
            
            // 初始化黑棋的炮
            pieces[2][7] = WrapPieces {
                id: cal_idx(2,7),
                point: Point::new(2,7),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Cannon,
                }),
            };

            // 初始化红棋的炮
            pieces[7][1] = WrapPieces {
                id: cal_idx(7,1),
                point: Point::new(7,1),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Cannon,
                }),
            };
            
            // 初始化红棋的炮
            pieces[7][7] = WrapPieces {
                id: cal_idx(7,7),
                point: Point::new(7,7),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Cannon,
                }),
            };

            // 初始化黑棋的车
            pieces[0][0] = WrapPieces {
                id: cal_idx(0, 0),
                point: Point::new(0, 0),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Chariot,
                }),
            };

            // 初始化黑棋的车
            pieces[0][8] = WrapPieces {
                id: cal_idx(0, 8),
                point: Point::new(0, 8),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Chariot,
                }),
            };
            
            // 初始化红棋的车
            pieces[9][0] = WrapPieces {
                id: cal_idx(9, 0),
                point: Point::new(9, 0),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Chariot,
                }),
            };

            // 初始化红棋的车
            pieces[9][8] = WrapPieces {
                id: cal_idx(9, 8),
                point: Point::new(9, 8),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Chariot,
                }),
            };

            // 初始化黑棋的马
            pieces[0][1] = WrapPieces {
                id: cal_idx(0, 1),
                point: Point::new(0, 1),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Horse,
                }),
            };

            // 初始化黑棋的马
            pieces[0][7] = WrapPieces {
                id: cal_idx(0, 7),
                point: Point::new(0, 7),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Horse,
                }),
            };

            // 初始化红棋的马
            pieces[9][1] = WrapPieces {
                id: cal_idx(9, 1),
                point: Point::new(9, 1),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Horse,
                }),
            };

            // 初始化红棋的马
            pieces[9][7] = WrapPieces {
                id: cal_idx(9, 7),
                point: Point::new(9, 7),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Horse,
                }),
            };

            // 初始化黑棋的相
            pieces[0][2] = WrapPieces {
                id: cal_idx(0, 2),
                point: Point::new(0, 2),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Elephant,
                }),
            };

            // 初始化黑棋的相
            pieces[0][6] = WrapPieces {
                id: cal_idx(0, 6),
                point: Point::new(0, 6),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Elephant,
                }),
            };

            // 初始化红棋的相
            pieces[9][2] = WrapPieces {
                id: cal_idx(9, 2),
                point: Point::new(9, 2),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Elephant,
                }),
            };
            
            // 初始化红棋的相
            pieces[9][6] = WrapPieces {
                id: cal_idx(9, 6),
                point: Point::new(9, 6),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Elephant,
                }),
            };

            // 初始化黑棋的士
            pieces[0][3] = WrapPieces {
                id: cal_idx(0, 3),
                point: Point::new(0, 3),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Advisor,
                }),
            };

            // 初始化黑棋的士
            pieces[0][5] = WrapPieces {
                id: cal_idx(0, 5),
                point: Point::new(0, 5),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::Advisor,
                }),
            };

            // 初始化红棋的士
            pieces[9][3] = WrapPieces {
                id: cal_idx(9, 3),
                point: Point::new(9, 3),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Advisor,
                }),
            };

            // 初始化红棋的士
            pieces[9][5] = WrapPieces {
                id: cal_idx(9, 5),
                point: Point::new(9, 5),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::Advisor,
                }),
            };

            // 初始化黑棋的将
            pieces[0][4] = WrapPieces {
                id: cal_idx(0, 4),
                point: Point::new(0, 4),
                priece: Some(Piece {
                    color: Color::Black,
                    piece_type: PieceType::King,
                }),
            };
            // 初始化红棋的帅
            pieces[9][4] = WrapPieces {
                id: cal_idx(9, 4),
                point: Point::new(9, 4),
                priece: Some(Piece {
                    color: Color::Red,
                    piece_type: PieceType::King,
                }),
            };
        }
        Self { pieces }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Piece> {
        self.pieces[x][y].priece.as_ref()
    }

    pub fn move_piece(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let piece = self.pieces[x1][y1].priece.take();
        self.pieces[x2][y2] = WrapPieces {
            id: cal_idx(x2, y2),
            point: Point::new(x2, y2),
            priece: piece,
        };
    }

    pub fn inner(self) -> Pieces {
        self.pieces
    }

    pub fn to_vec(self) -> Vec<WrapPieces> {
        let mut result = vec![];
        for i in 0..COLUMN_NUM {
            for j in 0..ROW_NUM {
                result.push(self.pieces[i][j]);
            }
        }
        result
    }
}