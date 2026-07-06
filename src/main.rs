use crate::game::plugins::WorldPlugin;
use bevy::prelude::*;

mod game;
mod pcg;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .run();
}
