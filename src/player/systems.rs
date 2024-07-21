use std::time::Duration;

use bevy::prelude::*;


use super::PLAYER_SPEED;
use super::components::*;
use crate::animation::components::{SpriteAnimationIndices, SpriteAnimationTimer};

use crate::collision::components::TargetPosition;
use crate::text_tilemap::components::Solid;
use crate::{TILE_SIZE, TILE_SCALE};


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((SpriteBundle {
            texture: asset_server.load("howard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE * TILE_SCALE, TILE_SIZE * TILE_SCALE)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
            ..default()
        },
        Player {},
        PlayerState::STANDING,
        DirectionX::RIGHT,
        TargetPosition {
            target: IVec2::default()
        },
        SpriteAnimationTimer {
            timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating)
        },
        SpriteAnimationIndices {
            start: 0,
            end: 1
        }
    ));
}



pub fn set_player_target_pos(
    mut player_query: Query<(&mut TargetPosition, &mut PlayerState), With<Player>>,
    button_input: Res<ButtonInput<KeyCode>>
) {
    let Ok((mut player_target_pos, mut player_state)) = player_query.get_single_mut() else { return; };

    if *player_state == PlayerState::MOVING { return; }

    if button_input.just_pressed(KeyCode::KeyD) {
        player_target_pos.target.x += TILE_SIZE as i32 * TILE_SCALE as i32;
        *player_state = PlayerState::MOVING;
    } else if button_input.just_pressed(KeyCode::KeyA) {
        player_target_pos.target.x -= TILE_SIZE as i32 * TILE_SCALE as i32;
        *player_state = PlayerState::MOVING;
    } else if button_input.just_pressed(KeyCode::KeyW) {
        player_target_pos.target.y += TILE_SIZE as i32 * TILE_SCALE as i32;
        *player_state = PlayerState::MOVING;
    } else if button_input.just_pressed(KeyCode::KeyS) {
        player_target_pos.target.y -= TILE_SIZE as i32 * TILE_SCALE as i32;
        *player_state = PlayerState::MOVING;
    }
}


pub fn move_player(
    mut player_query: Query<(&mut Transform, &TargetPosition, &PlayerState), With<Player>>
) {
    let Ok((mut player_transform, player_target_pos, player_state)) = player_query.get_single_mut() else { return; };


    if !(*player_state == PlayerState::MOVING) { return; }
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    let player_z = player_transform.translation.z;
    player_transform.translation = Vec3::new(player_x.lerp(player_target_pos.target.x as f32, PLAYER_SPEED), player_y.lerp(player_target_pos.target.y as f32, PLAYER_SPEED), player_z);
}


pub fn stop_player(
    mut player_query: Query<(&Transform, &TargetPosition, &mut PlayerState), With<Player>>
) {
    let Ok((player_transform, player_target_pos, mut player_state)) = player_query.get_single_mut() else { return; };

    if player_target_pos.target == IVec2::new(player_transform.translation.x.round() as i32, player_transform.translation.y.round() as i32) {
        *player_state = PlayerState::STANDING;
    } else {
        println!("{} should == {}", player_transform.translation.x.round() as i32, player_target_pos.target.x);
    }
}



pub fn player_hit_wall(
    mut player_query: Query<(&Transform, &mut TargetPosition, &mut PlayerState), With<Player>>,
    tile_query: Query<&TargetPosition, (With<Solid>, Without<Player>)>
) {
    let Ok((player_transform, mut player_target_pos, mut player_state)) = player_query.get_single_mut() else { return; };

    for tile_target_pos in tile_query.iter() {
        if player_target_pos.collide(tile_target_pos) {
            player_target_pos.target = IVec2::new(player_transform.translation.x.round() as i32, player_transform.translation.y.round() as i32);
            *player_state = PlayerState::STANDING;
            println!("collided!");
            return;
        }
    }
}