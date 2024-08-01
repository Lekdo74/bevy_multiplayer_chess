use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{piece::*, state::GameState, GlobalTextureAtlas, ResizeCooldownTimer, SPRITE_W};

// .add_systems(OnEnter(GameState::GameInitResources), setup_board_resource);

#[derive(Debug, Resource)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

pub struct BoardPlugin;

#[derive(Component)]
struct BoardEntity;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInitEntities), init_board)
            .add_systems(Update, resize_board.run_if(in_state(GameState::InGame)));
    }
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

        Self { pieces }
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
        print!("{} ", row_label);

        print!("|");

        for (_, cell) in row.iter().enumerate() {
            let cell_str = match cell {
                Some(piece) => piece.to_string(),
                None => " ".to_string(),
            };

            print!(" {} |", cell_str);
        }
        println!();

        print!("   ");
        for i in 0..row.len() {
            print!("---");
            if i < row.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    print!("   ");
    for col in 'a'..='h' {
        print!(" {} ", col);
        if col != 'h' {
            print!(" ");
        }
    }
    println!();
}

fn init_board(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    board: Res<Board>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if window_query.is_empty() {
        return;
    }

    let window = window_query.single();

    let (width, height) = (window.width(), window.height());

    display_board(&mut commands, &handle, &board, width, height);

    next_state.set(GameState::InGame);
}

fn display_board(
    commands: &mut Commands,
    handle: &Res<GlobalTextureAtlas>,
    board: &Res<Board>,
    width: f32,
    height: f32,
) {
    let board_origin = Vec2::new(-width / 2.0, height / 2.0);
    let cell_size = (width.min(height)) / 8.0;
    let half_cell_size = cell_size / 2.0;

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        board_origin.x + half_cell_size + (cell_size * i as f32),
                        board_origin.y - half_cell_size - (cell_size * j as f32),
                        0.0,
                    ))
                    .with_scale(Vec3::splat(cell_size / SPRITE_W as f32)),
                    texture: handle.image.clone().unwrap(),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: if (i + j) % 2 == 0 { 12 } else { 13 },
                },
                BoardEntity,
            ));
        }
    }
}

fn resize_board(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    board: Res<Board>,
    mut resize_events: EventReader<WindowResized>,
    board_entities: Query<Entity, With<BoardEntity>>,
) {
    let events: Vec<&WindowResized> = resize_events.read().collect();

    if events.is_empty() {
        return;
    }

    let last_event = events.last();

    let (width, height) = (last_event.unwrap().width, last_event.unwrap().height);

    for entity in board_entities.iter() {
        commands.entity(entity).despawn();
    }

    display_board(&mut commands, &handle, &board, width, height);
}
