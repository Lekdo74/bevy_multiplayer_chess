use std::fmt;

use bevy::prelude::*;

use crate::{
    board::{print_board, Board, BoardConfiguration, PieceEntity},
    state::GameState,
    CursorPosition, SPRITE_SHEET_W,
};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_dragging,
                handle_dragging,
                handle_drop.after(handle_dragging),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub index: usize,
}

#[derive(Component)]
pub struct Dragging {
    init_x: usize,
    init_y: usize,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_char = match self {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
        };
        write!(f, "{}", piece_char)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_rep = format!("{}", self.piece_type);
        if self.color == PieceColor::White {
            write!(f, "{}", piece_rep.to_uppercase())
        } else {
            write!(f, "{}", piece_rep.to_lowercase())
        }
    }
}

pub fn get_piece_index(piece_type: PieceType, color: PieceColor) -> usize {
    let mut index = match piece_type {
        PieceType::King => 0,
        PieceType::Queen => 1,
        PieceType::Rook => 2,
        PieceType::Bishop => 3,
        PieceType::Knight => 4,
        PieceType::Pawn => 5,
    };

    if color == PieceColor::White {
        index += SPRITE_SHEET_W;
    }

    return index as usize;
}

fn add_dragging(
    mut commands: Commands,
    board_config: Res<BoardConfiguration>,
    mut piece_query: Query<(&Transform, Entity), With<PieceEntity>>,
    piece_dragged_query: Query<(&Transform, Entity), (With<PieceEntity>, With<Dragging>)>,
    cursor_position: Res<CursorPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if !piece_dragged_query.is_empty() {
        return;
    }

    if piece_query.is_empty() {
        return;
    }

    if let Some(cursor_position) = cursor_position.position {
        for (transform_piece, entity_piece) in piece_query.iter_mut() {
            if transform_piece.translation.x
                > cursor_position.x - board_config.half_cell_size as f32
                && transform_piece.translation.x
                    < cursor_position.x + board_config.half_cell_size as f32
                && transform_piece.translation.y
                    < cursor_position.y + board_config.half_cell_size as f32
                && transform_piece.translation.y
                    > cursor_position.y - board_config.half_cell_size as f32
            {
                let board_x = (((-board_config.board_origin.x + cursor_position.x)
                    / board_config.cell_size) as usize)
                    .clamp(0, 7);
                let board_y = (((-board_config.board_origin.y + cursor_position.y)
                    / -board_config.cell_size) as usize)
                    .clamp(0, 7);

                commands.entity(entity_piece).insert(Dragging {
                    init_x: board_x,
                    init_y: board_y,
                });

                return;
            }
        }
    }
}

fn handle_dragging(
    mut dragging_query: Query<(&mut Transform, Entity), With<Dragging>>,
    cursor_position: Res<CursorPosition>,
) {
    if dragging_query.is_empty() {
        return;
    }

    if let Some(cursor_position) = cursor_position.position {
        for (mut transform_dragging, _) in dragging_query.iter_mut() {
            transform_dragging.translation = Vec3::new(cursor_position.x, cursor_position.y, 2.0);
        }
    }
}

fn handle_drop(
    mut commands: Commands,
    board_config: Res<BoardConfiguration>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut dragging_query: Query<(&mut Transform, Entity, &Dragging), With<Dragging>>,
    cursor_position: Res<CursorPosition>,
    mut board: ResMut<Board>,
    mut piece_query: Query<(&Transform, Entity), (With<PieceEntity>, Without<Dragging>)>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (mut transform_dragging, entity_piece, dragging) in dragging_query.iter_mut() {
        let board_x = (((-board_config.board_origin.x + cursor_position.position.unwrap().x)
            / board_config.cell_size) as usize)
            .clamp(0, 7);
        let board_y = (((-board_config.board_origin.y + cursor_position.position.unwrap().y)
            / -board_config.cell_size) as usize)
            .clamp(0, 7);

        transform_dragging.translation = Vec3::new(board_config.board_origin.x + board_x as f32 * board_config.cell_size + board_config.half_cell_size, board_config.board_origin.y - board_y as f32 * board_config.cell_size - board_config.half_cell_size, 1.0);


        board.pieces[board_y][board_x].take();

        if let Some(cursor_position) = cursor_position.position {
            for (transform_piece, entity_piece) in piece_query.iter_mut() {
                if transform_piece.translation.x
                    > cursor_position.x - board_config.half_cell_size as f32
                    && transform_piece.translation.x
                        < cursor_position.x + board_config.half_cell_size as f32
                    && transform_piece.translation.y
                        < cursor_position.y + board_config.half_cell_size as f32
                    && transform_piece.translation.y
                        > cursor_position.y - board_config.half_cell_size as f32
                {
                    commands.entity(entity_piece).despawn();
                }
            }
        }

        if let Some(piece) = board.pieces[dragging.init_y][dragging.init_x].take() {
            board.pieces[board_y][board_x] = Some(piece);
        }

        commands.entity(entity_piece).remove::<Dragging>();

        print_board(&board);
    }
}
