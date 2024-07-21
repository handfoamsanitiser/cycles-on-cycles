#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::window::WindowResolution;
use bevy::prelude::*;

const TILE_SIZE: f32 = 16.0;
const PLAYER_SCALE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {

            meta_check: AssetMetaCheck::Never,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800.0, 800.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())
    )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("howard.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_SIZE * PLAYER_SCALE, TILE_SIZE * PLAYER_SCALE)),
            ..default()
        },
        ..default()
    });
}
