use crate::{
    game::{
        components::{Movable, ObjectOnGrid},
        constants,
        player::components::Player,
    },
    pcg::{terrain, terrain::tile},
};
use bevy::prelude::*;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};
use std::io::{Write, stdout};

pub fn render_ascii(
    terrain: Res<terrain::resources::TerrainWorld>,
    player_query: Query<
        &Transform,
        (
            With<ObjectOnGrid>,
            With<Player>,
            With<Movable>,
            Changed<Transform>,
        ),
    >,
) {
    let mut so = stdout();

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let (chunk_coord, local_tile_coord) =
        terrain::utils::pos_to_cell_world(player_transform.translation, &terrain);

    let world_ivec2: IVec2 = terrain::utils::get_world_ivec2(chunk_coord, local_tile_coord);

    // compute ascii camera rect
    let camera_width: u32 = constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.x;
    let camera_height: u32 = constants::ASCII_CAMERA_SIZE * constants::ASCII_CAMERA_ASPECT_RATIO.y;
    let camera_extent: UVec2 = UVec2 {
        x: camera_width / 2,
        y: camera_height / 2,
    };

    let Ok(_cmd) = so.execute(terminal::Clear(terminal::ClearType::All)) else {
        println!("Failed to clear terminal");
        return;
    };

    let Ok(_cmd) = so.execute(cursor::MoveTo(0, 0)) else {
        println!("Failed to move cursor to top left");
        return;
    };

    // let mut output: Vec<String> = Vec::new();
    let lower_left: IVec2 = IVec2 {
        x: world_ivec2.x - camera_extent.x as i32,
        y: world_ivec2.y - camera_extent.y as i32,
    };

    let upper_right: IVec2 = IVec2 {
        x: world_ivec2.x + camera_extent.x as i32,
        y: world_ivec2.y + camera_extent.y as i32,
    };
    for y in (lower_left.y)..=(upper_right.y) {
        // let mut line: String = String::new();
        for x in (lower_left.x)..=(upper_right.x) {
            let tile_char: char = terrain
                .tile_at_world_ivec2(IVec2 { x, y })
                .map_or(' ', |t| tile::tile_appearance_ascii(t));

            // line.push(tile_char);
            let local_x = (x - lower_left.x) as u16;
            let local_y = (upper_right.y - y) as u16; // invert y for terminal coordinates
            so.queue(cursor::MoveTo(local_x, local_y))
                .unwrap()
                .queue(style::PrintStyledContent(tile_char.white()))
                .unwrap();
        }
        // output.push(line);
    }

    // reverse the print as first row in the data is the bottom row in the render
    // for line in output.iter().rev() {
    //     println!("{}", line);
    // }
}

pub fn handle_terminal_player_movement(
    terrain: Res<terrain::resources::TerrainWorld>,
    player_query: Query<
        &Transform,
        (
            With<ObjectOnGrid>,
            With<Player>,
            With<Movable>,
            Changed<Transform>,
        ),
    >,
) {
}

// pub fn handle_player_movement(
//     time: Res<Time>,
//     keyboard_input: Option<Res<ButtonInput<KeyCode>>>,
//     terrain: Res<terrain::resources::TerrainWorld>,
//     query: Query<(&mut Transform, &mut Movable, &mut ObjectOnGrid), With<Player>>,
// ) {
//     let Some(keyboard_input) = keyboard_input else {
//         return;
//     };
//
//     let mut direction: Vec2 = Vec2::ZERO;
//
//     if keyboard_input.pressed(KeyCode::KeyW) {
//         direction.y = 1.0;
//     } else if keyboard_input.pressed(KeyCode::KeyS) {
//         direction.y = -1.0;
//     } else if keyboard_input.pressed(KeyCode::KeyA) {
//         direction.x = -1.0;
//     } else if keyboard_input.pressed(KeyCode::KeyD) {
//         direction.x = 1.0;
//     }
//
//     direction = direction.normalize_or_zero();
//
//     if direction == Vec2::ZERO {
//         return;
//     }
//
//     for (mut transform, mut movable, mut object_on_grid) in query {
//         if keyboard_input.just_released(KeyCode::KeyW)
//             || keyboard_input.just_released(KeyCode::KeyS)
//             || keyboard_input.just_released(KeyCode::KeyA)
//             || keyboard_input.just_released(KeyCode::KeyD)
//         {
//             movable.last_step_time = None;
//         }
//
//         // let terrain_chunk: &terrain::resources::TerrainChunk =
//         //     terrain.chunk_of_pos(transform.translation).1.unwrap();
//         let move_interval: f32 = movable.move_interval();
//
//         // if last step time is reset or enough time has passed since the last step
//         let move_not_in_cooldown = movable.last_step_time.map_or(true, |last_time| {
//             time.elapsed_secs_f64() - last_time >= move_interval as f64
//         });
//
//         // precompute the next grid pos
//         let next_grid_pos: Vec3 = terrain::utils::cell_round(
//             Vec3 {
//                 x: direction.x * terrain::constants::TILE_SIZE,
//                 y: direction.y * terrain::constants::TILE_SIZE,
//                 z: 0.0,
//             } + object_on_grid.internal_translation,
//         );
//
//         let (chunk_coord, next_cell_coord) =
//             terrain::utils::pos_to_cell_world(next_grid_pos, &terrain);
//
//         let is_tile_walkable = terrain
//             .is_tile_walkable(chunk_coord, next_cell_coord)
//             .unwrap_or(false);
//
//         // execute the move only if the move is not in cooldown and the next tile is walkable
//         if move_not_in_cooldown && is_tile_walkable {
//             // update the step time to restart cooldown
//             if next_grid_pos != transform.translation {
//                 movable.last_step_time = Some(time.elapsed_secs_f64());
//             }
//
//             object_on_grid.internal_translation = next_grid_pos;
//             transform.translation = next_grid_pos;
//         }
//     }
// }
