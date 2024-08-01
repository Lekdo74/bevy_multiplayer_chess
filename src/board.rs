use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{piece::{self, *}, state::GameState, GlobalTextureAtlas, SPRITE_W};

#[derive(Debug, Resource)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

pub struct BoardPlugin;

#[derive(Resource)]
pub struct BoardConfiguration {
    pub board_origin: Vec2,
    pub cell_size: f32,
    pub half_cell_size: f32,
}

#[derive(Component)]
struct BoardEntity;

#[derive(Component)]
pub struct PieceEntity;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInitEntities), init_board)
            .add_systems(Update, resize_board.run_if(in_state(GameState::InGame)));
    }
}

impl Default for BoardConfiguration {
    fn default() -> Self {
        Self {
            board_origin: Vec2::ZERO,
            cell_size: 0.0,
            half_cell_size: 0.0,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let piece_rows = [(PieceColor::Black, 0), (PieceColor::White, 7)];

        for &(color, row) in piece_rows.iter() {
            pieces[row][0] = Some(Piece {
                piece_type: PieceType::Rook,
                color,
                index: get_piece_index(PieceType::Rook, color),
            });
            pieces[row][1] = Some(Piece {
                piece_type: PieceType::Knight,
                color,
                index: get_piece_index(PieceType::Knight, color),
            });
            pieces[row][2] = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
                index: get_piece_index(PieceType::Bishop, color),
            });
            pieces[row][3] = Some(Piece {
                piece_type: PieceType::Queen,
                color,
                index: get_piece_index(PieceType::Queen, color),
            });
            pieces[row][4] = Some(Piece {
                piece_type: PieceType::King,
                color,
                index: get_piece_index(PieceType::King, color),
            });
            pieces[row][5] = Some(Piece {
                piece_type: PieceType::Bishop,
                color,
                index: get_piece_index(PieceType::Bishop, color),
            });
            pieces[row][6] = Some(Piece {
                piece_type: PieceType::Knight,
                color,
                index: get_piece_index(PieceType::Knight, color),
            });
            pieces[row][7] = Some(Piece {
                piece_type: PieceType::Rook,
                color,
                index: get_piece_index(PieceType::Rook, color),
            });

            let pawn_row = if color == PieceColor::Black { 1 } else { 6 };
            for col in 0..8 {
                pieces[pawn_row][col] = Some(Piece {
                    piece_type: PieceType::Pawn,
                    color,
                    index: get_piece_index(PieceType::Pawn, color),
                });
            }
        }

        // let board = Self { pieces };
        // print_board(&board);
        // board

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

    for (i, row) in board.pieces.iter().enumerate() {
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
    mut board_configuration: ResMut<BoardConfiguration>,
    board_entities: Query<Entity, With<BoardEntity>>,
) {
    if window_query.is_empty() {
        return;
    }

    let window = window_query.single();

    let (width, height) = (window.width(), window.height());

    create_board(&mut commands, &handle, &board, width, height, board_configuration, board_entities);

    next_state.set(GameState::InGame);
}

fn create_board(
    commands: &mut Commands,
    handle: &Res<GlobalTextureAtlas>,
    board: &Res<Board>,
    width: f32,
    height: f32,
    mut board_configuration: ResMut<BoardConfiguration>,
    board_entities: Query<Entity, With<BoardEntity>>,
) {
    for entity in board_entities.iter() {
        commands.entity(entity).despawn();
    }

    let board_origin = Vec2::new(-width / 2.0, height / 2.0);
    let cell_size = (width.min(height)) / 8.0;
    let half_cell_size = cell_size / 2.0;

    board_configuration.board_origin = board_origin;
    board_configuration.cell_size = cell_size;
    board_configuration.half_cell_size = half_cell_size;

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

    let piece_size = cell_size / SPRITE_W as f32;

    for (i, row) in board.pieces.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Some(piece) = cell {
                let piece_pos_x = board_origin.x + half_cell_size + (cell_size * j as f32);
                let piece_pos_y = board_origin.y - half_cell_size - (cell_size * i as f32);

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(
                            piece_pos_x,
                            piece_pos_y,
                            1.0,
                        ))
                        .with_scale(Vec3::splat(piece_size)),
                        texture: handle.image.clone().unwrap(),
                        ..Default::default()
                    },
                    TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: piece.index,
                    },
                    BoardEntity,
                    PieceEntity
                ));
            }
        }
    }
}

fn resize_board(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    board: Res<Board>,
    mut resize_events: EventReader<WindowResized>,
    mut board_configuration: ResMut<BoardConfiguration>,
    board_entities: Query<Entity, With<BoardEntity>>,
) {
    let events: Vec<&WindowResized> = resize_events.read().collect();

    if events.is_empty() {
        return;
    }

    let last_event = events.last();

    let (width, height) = (last_event.unwrap().width, last_event.unwrap().height);

    create_board(&mut commands, &handle, &board, width, height, board_configuration, board_entities);
}