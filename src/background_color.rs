use bevy::prelude::*;

use crate::{state::GameState, BG_COLOR};

pub struct BackgroundColorPlugin;

impl Plugin for BackgroundColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_background_color);
    }
}

fn setup_background_color(mut commands: Commands) {
    println!("Setting up background color to: ({}, {}, {})", BG_COLOR.0, BG_COLOR.1, BG_COLOR.2);
    commands.insert_resource(ClearColor(Color::srgb_u8(
        BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
    )));
}