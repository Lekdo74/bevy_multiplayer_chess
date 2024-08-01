use bevy::prelude::*;
use bevy_multiplayer_chess::{
    board::BoardPlugin, camera::MyCameraPlugin, close_on_esc::CloseOnEscapePlugin, default_plugins::MyDefaultPlugins, resources::ResourcesPlugin, state::GameState
};

fn main() {
    App::new()
        .add_plugins(MyDefaultPlugins)
        .add_plugins(CloseOnEscapePlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(BoardPlugin)
        .init_state::<GameState>()
        .run();
}
