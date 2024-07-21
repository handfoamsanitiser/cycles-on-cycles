use bevy::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::collision::components::TargetPosition;
use crate::{TILE_SIZE, TILE_SCALE};
use super::components::Solid;
use super::resources::Level;

const ROOT_TEXT_PATH: &str = "assets/text_tilemaps/";
const ROOT_SPRITE_PATH: &str = "tilemaps/";



pub fn load_collider_file(path: &str, width: usize, height: usize) -> Option<Level> {
    let file = File::open(format!("{}{}", ROOT_TEXT_PATH, path)).expect("blud where is the file");

    let mut level = Level::new(width, height);

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let Ok(line) = line else { return None; };

        for (x, char) in line.chars().enumerate() {
            level.level[y][x] = char;
        }
    }

    level.print_level();
    Some(level)
}



pub fn spawn_level(mut commands: &mut Commands, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>, level: &Level, texture_path: &str) {
    let texture: Handle<Image> = asset_server.load(format!("{}{}", ROOT_SPRITE_PATH, texture_path));
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for y in 0..10 {
        for x in 0..10 {
            match level.level[y][x] {
                '0' => spawn_solid_tile(&mut commands, texture.clone(), texture_atlas_layout.clone(), 0, Vec2::new(x as f32, y as f32)),
                _ => ()
            }
        }
    }
}


// Spawns a tile with colliders
pub fn spawn_solid_tile(commands: &mut Commands, texture: Handle<Image>, layout: Handle<TextureAtlasLayout>, index: usize, pos: Vec2) {
    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(pos.x * TILE_SIZE * TILE_SCALE, pos.y * TILE_SIZE * TILE_SCALE, 0.0),
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE * TILE_SCALE, TILE_SIZE * TILE_SCALE)),
                ..default()
            },
            ..default()
        }, 
        TextureAtlas {
            layout,
            index
        },
        TargetPosition {
            target: IVec2::new((pos.x * TILE_SIZE * TILE_SCALE) as i32, (pos.y * TILE_SIZE * TILE_SCALE) as i32)
        },
        Solid {}
    ));
}


// Spawns a tile without colliders
/*pub fn spawn_walkable_tile() {

}*/


