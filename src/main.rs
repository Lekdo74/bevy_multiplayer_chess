use bevy::prelude::*;
use bevy_multiplayer_chess::{ressources::ResourcesPlugin, camera::MyCameraPlugin, close_on_esc::CloseOnEscapePlugin, state::GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CloseOnEscapePlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .init_state::<GameState>()
        .run();
}
