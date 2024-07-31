use bevy::prelude::*;

use crate::{piece::*, state::GameState, BG_COLOR};

pub struct ResourcesPlugin;

#[derive(Debug, Resource)]
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_background_color)
            .insert_resource(Board::default());
    }
}

fn setup_background_color(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb_u8(
        BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
    )));
}

impl Default for Board {
    fn default() -> Self {
        let mut cells = Vec::new();

        let piece_rows = [(PieceColor::White, 0), (PieceColor::Black, 7)];

        for (color, row) in piece_rows.iter() {
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Rook,
                    color: *color,
                },
                position: (0, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Knight,
                    color: *color,
                },
                position: (1, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Bishop,
                    color: *color,
                },
                position: (2, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Queen,
                    color: *color,
                },
                position: (3, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::King,
                    color: *color,
                },
                position: (4, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Bishop,
                    color: *color,
                },
                position: (5, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Knight,
                    color: *color,
                },
                position: (6, *row),
            });
            cells.push(Cell {
                piece: Piece {
                    piece_type: PieceType::Rook,
                    color: *color,
                },
                position: (7, *row),
            });

            // Pawns
            for col in 0..8 {
                cells.push(Cell {
                    piece: Piece {
                        piece_type: PieceType::Pawn,
                        color: *color,
                    },
                    position: (col, if *color == PieceColor::White { 1 } else { 6 }),
                });
            }
        }

        println!();
        print_board(&cells);

        Self { cells }
    }
}

pub fn print_board(cells: &Vec<Cell>) {
    let mut board: Vec<Vec<String>> = vec![vec![" ".to_string(); 8]; 8];

    for cell in cells {
        let (x, y) = cell.position;
        let piece_rep = format!("{}", cell.piece);
        board[y][x] = piece_rep;
    }

    for row in board.iter().rev() {
        println!("{:?}", row);
    }
}
