use bevy::prelude::*;

pub const DEFAULT_GRID_WIDTH: usize = 64;
pub const DEFAULT_GRID_HEIGHT: usize = 64;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_DIMESNION: Vec2 = Vec2::splat(TILE_SIZE);
pub const DEFAULT_GRID_CENTER: Vec2 = Vec2 {
    x: (DEFAULT_GRID_WIDTH as f32 * TILE_SIZE) / 2.0,
    y: (DEFAULT_GRID_HEIGHT as f32 * TILE_SIZE) / 2.0,
};
