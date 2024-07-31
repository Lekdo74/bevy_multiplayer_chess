use bevy::prelude::*;

use crate::{piece::*, state::GameState, BG_COLOR};

pub struct ResourcesPlugin;

#[derive(Debug, Resource)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
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
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let piece_rows = [(PieceColor::White, 0), (PieceColor::Black, 7)];

        for &(color, row) in piece_rows.iter() {
            pieces[row][0] = Some(Piece {
                piece_type: PieceType::Rook,
                color,
            });
            pieces[row][1] = Some(Piece {
                piece_type: PieceType::Knight,
                color,
            });
            pieces[row][2] = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
            });
            pieces[row][3] = Some(Piece {
                piece_type: PieceType::Queen,
                color,
            });
            pieces[row][4] = Some(Piece {
                piece_type: PieceType::King,
                color,
            });
            pieces[row][5] = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
            });
            pieces[row][6] = Some(Piece {
                piece_type: PieceType::Knight,
                color,
            });
            pieces[row][7] = Some(Piece {
                piece_type: PieceType::Rook,
                color,
            });

            let pawn_row = if color == PieceColor::White { 1 } else { 6 };
            for col in 0..8 {
                pieces[pawn_row][col] = Some(Piece {
                    piece_type: PieceType::Pawn,
                    color,
                });
            }
        }

        let board = Self { pieces };
        print_board(&board);
        board
    }
}

pub fn print_board(board: &Board) {
    print!("   ");
    for i in 0..8 {
        print!("---");
        if i < 7 {
            print!(" ");
        }
    }
    println!();

    for (i, row) in board.pieces.iter().rev().enumerate() {
        let row_label = 8 - i;
        print!("{} ", row_label); // Print the row label

        print!("|"); // Start of row

        for (j, cell) in row.iter().enumerate() {
            let cell_str = match cell {
                Some(piece) => piece.to_string(),
                None => " ".to_string(),
            };

            print!(" {} |", cell_str);

            // if j < row.len() - 1 {
            //     print!(" "); // Add spacing between columns
            // }
        }
        println!(); // End of row

        // Print the separator line
        print!("   ");
        for i in 0..row.len() {
            print!("---");
            if i < row.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    // Print column labels
    print!("   ");
    for col in 'a'..='h' {
        print!(" {} ", col);
        if col != 'h' {
            print!(" ");
        }
    }
    println!();
}
