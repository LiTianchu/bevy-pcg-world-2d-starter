use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        player::components::Player,
    },
    pcg::terrain,
};
use bevy::prelude::*;

pub fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Transform, &Player, &Movable, &mut ObjectOnGrid)>,
) {
    let mut direction: Vec2 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y = 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y = -1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x = -1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x = 1.0;
    }

    direction = direction.normalize_or_zero();

    for (mut transform, _player, movable, mut object_on_grid) in query {
        object_on_grid.internal_translation.x += direction.x * movable.speed;
        object_on_grid.internal_translation.y += direction.y * movable.speed;
        transform.translation = terrain::utils::pos_to_grid(object_on_grid.internal_translation);
        println!("Player movement direction: {:?}", direction);
        println!("Player moved to position: {:?}", transform.translation);
    }
}
