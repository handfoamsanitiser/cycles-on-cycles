use bevy::prelude::*;


use super::components::*;
use crate::collision::components::TargetPosition;
use crate::text_tilemap::components::Solid;
use crate::{TILE_SIZE, TILE_SCALE};


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((SpriteBundle {
            texture: asset_server.load("howard.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE * TILE_SCALE, TILE_SIZE * TILE_SCALE)),
                ..default()
            },
            ..default()
        }, 
        Player {},
        TargetPosition {
            target: IVec2::default()
        }
    ));
}



pub fn move_player(
    mut player_query: Query<&mut TargetPosition, With<Player>>,
    button_input: Res<ButtonInput<KeyCode>>
) {
    let Ok(mut player_target_pos) = player_query.get_single_mut() else { return; };

    if button_input.just_pressed(KeyCode::KeyD) {
        player_target_pos.target.x += TILE_SIZE as i32 * TILE_SCALE as i32;
    } else if button_input.just_pressed(KeyCode::KeyA) {
        player_target_pos.target.x -= TILE_SIZE as i32 * TILE_SCALE as i32;
    } else if button_input.just_pressed(KeyCode::KeyW) {
        player_target_pos.target.y += TILE_SIZE as i32 * TILE_SCALE as i32;
    } else if button_input.just_pressed(KeyCode::KeyS) {
        player_target_pos.target.y -= TILE_SIZE as i32 * TILE_SCALE as i32;
    }

}



pub fn player_hit_wall(
    mut player_query: Query<(&mut Transform, &mut TargetPosition), With<Player>>,
    tile_query: Query<&TargetPosition, (With<Solid>, Without<Player>)>
) {
    let Ok((mut player_transform, mut player_target_pos)) = player_query.get_single_mut() else { return; };

    for tile_target_pos in tile_query.iter() {
        if player_target_pos.collide(tile_target_pos) {
            player_target_pos.target = IVec2::new(player_transform.translation.x as i32, player_transform.translation.y as i32);
            println!("collided!");
            return;
        }
    }

    let player_z = player_transform.translation.z;
    player_transform.translation = Vec3::new(player_target_pos.target.x as f32, player_target_pos.target.y as f32, player_z);
}