use bevy::prelude::*;
use bevy_multiplayer_chess::{
    camera::MyCameraPlugin, close_on_esc::CloseOnEscapePlugin, default_plugins::MyDefaultPlugins, ressources::ResourcesPlugin, state::GameState
};

fn main() {
    App::new()
        .add_plugins(MyDefaultPlugins)
        .add_plugins(CloseOnEscapePlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .init_state::<GameState>()
        .run();
}
