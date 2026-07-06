use crate::pcg::terrain;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection::default_2d()),
        Transform::from_xyz(
            terrain::constants::DEFAULT_GRID_CENTER.x,
            terrain::constants::DEFAULT_GRID_CENTER.y,
            100.0,
        ),
    ));
}
