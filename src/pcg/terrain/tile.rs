use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Void,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "W"),
            Tile::Floor => write!(f, "F"),
            Tile::Void => write!(f, "V"),
        }
    }
}

pub fn tile_appearance(tile: Tile) -> Color {
    let color: Color = match tile {
        Tile::Wall => Color::srgba(0.5, 0.4, 0.6, 1.0),
        Tile::Floor => Color::srgba(0.2, 0.4, 0.9, 1.0),
        Tile::Void => Color::srgba(0.0, 0.0, 0.0, 1.0),
    };
    return color;
}
