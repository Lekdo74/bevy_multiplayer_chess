//! Shows how to render simple primitive shapes with a single color.

use bevy::prelude::*;
use bevy_multiplayer_chess::close_on_esc::CloseOnEscapePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CloseOnEscapePlugin)
        .run();
}
