#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod dev_tools;
mod states;
mod player;
mod animation;
mod collision;
mod text_tilemap;


use states::{GameState, LevelState, TurnState};
use animation::MySpriteAnimationPlugin;
use player::PlayerPlugin;
use text_tilemap::helper::{load_collider_file, spawn_level};
use text_tilemap::resources::LevelManager;


pub const TILE_SIZE: f32 = 16.0;
pub const TILE_SCALE: f32 = 4.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::new(0.60784, 0.73725, 0.05882, 0.0))))
        .add_plugins(DefaultPlugins.set(AssetPlugin {

            meta_check: AssetMetaCheck::Never,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1024.0, 1024.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())

        .init_resource::<LevelManager>()

        .add_plugins(MySpriteAnimationPlugin)
        .add_plugins(PlayerPlugin)

        .add_systems(Startup, setup)
        .add_systems(Startup, load_test_level)

        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(TILE_SIZE * TILE_SCALE / 2.0, TILE_SIZE * TILE_SCALE / 2.0, 5.0),
        ..default()
    });
}

fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let file = load_collider_file("test.txt", 10, 10).unwrap();
    spawn_level(&mut commands, &asset_server, &mut texture_atlas_layouts, &file, "test.png");
}
