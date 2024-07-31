use bevy::prelude::*;
use bevy_multiplayer_chess::{background_color::BackgroundColorPlugin, camera::MyCameraPlugin, close_on_esc::CloseOnEscapePlugin, state::GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CloseOnEscapePlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(BackgroundColorPlugin)
        .init_state::<GameState>()
        .run();
}
