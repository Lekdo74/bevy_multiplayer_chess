use std::time::Duration;

use bevy::prelude::*;

use crate::{
    board::Board, state::GameState, BG_COLOR, SPRITE_H, SPRITE_SHEET_H, SPRITE_SHEET_PATH,
    SPRITE_SHEET_W, SPRITE_W,
};

pub struct ResourcesPlugin;

#[derive(Resource)]
struct LoadCompletion {
    setup_background_color: bool,
    load_assets: bool,
}

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct ResizeCooldownTimer(pub Timer);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadCompletion::default())
            .insert_resource(GlobalTextureAtlas::default())
            .insert_resource(ResizeCooldownTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Loading), setup_background_color)
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                check_load_completion.run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnEnter(GameState::GameInitResources), setup_board_resource);
    }
}

fn setup_background_color(mut commands: Commands, mut load_completion: ResMut<LoadCompletion>) {
    commands.insert_resource(ClearColor(Color::srgb_u8(
        BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
    )));

    load_completion.setup_background_color = true;
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut load_completion: ResMut<LoadCompletion>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_W, SPRITE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    load_completion.load_assets = true;
}

fn check_load_completion(
    load_completion: Res<LoadCompletion>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if load_completion.setup_background_color && load_completion.load_assets {
        next_state.set(GameState::GameInitResources);
    }
}

fn setup_board_resource(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(Board::default());

    next_state.set(GameState::GameInitEntities);
}

impl Default for LoadCompletion {
    fn default() -> Self {
        Self {
            setup_background_color: false,
            load_assets: false,
        }
    }
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}
