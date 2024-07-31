use bevy::prelude::*;

use crate::{piece::*, state::GameState, BG_COLOR};

pub struct ResourcesPlugin;

#[derive(Debug, Resource)]
pub struct BoardState {
    pub pieces: Vec<PiecePosition>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_background_color)
            .insert_resource(BoardState::default());
    }
}

fn setup_background_color(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb_u8(
        BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
    )));
}

impl Default for BoardState {
    fn default() -> Self {
        let mut pieces = Vec::new();

        let piece_rows = [(PieceColor::White, 0), (PieceColor::Black, 7)];

        for (color, row) in piece_rows.iter() {
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Rook,
                    color: *color,
                },
                position: (0, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Knight,
                    color: *color,
                },
                position: (1, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Bishop,
                    color: *color,
                },
                position: (2, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Queen,
                    color: *color,
                },
                position: (3, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::King,
                    color: *color,
                },
                position: (4, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Bishop,
                    color: *color,
                },
                position: (5, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Knight,
                    color: *color,
                },
                position: (6, *row),
            });
            pieces.push(PiecePosition {
                piece: Piece {
                    piece_type: PieceType::Rook,
                    color: *color,
                },
                position: (7, *row),
            });

            // Pawns
            for col in 0..8 {
                pieces.push(PiecePosition {
                    piece: Piece {
                        piece_type: PieceType::Pawn,
                        color: *color,
                    },
                    position: (col, if *color == PieceColor::White { 1 } else { 6 }),
                });
            }
        }

        println!();
        print_board(&pieces);

        Self { pieces }
    }
}

pub fn print_board(pieces: &Vec<PiecePosition>) {
    let mut board: Vec<Vec<String>> = vec![vec![" ".to_string(); 8]; 8];

    for pos in pieces {
        let (x, y) = pos.position;
        let piece_rep = format!("{}", pos.piece);
        board[y][x] = piece_rep;
    }

    // Print the board
    for row in board.iter().rev() {
        println!("{:?}", row);
    }
}