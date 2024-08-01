use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};

pub struct MyDefaultPlugins;

impl Plugin for MyDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                // .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 800.0).into(),
                        resizable: true,
                        focused: true,
                        title: "Chess".to_string(),
                        // mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        );
        // .add_systems(Startup, center_window);
    }
}