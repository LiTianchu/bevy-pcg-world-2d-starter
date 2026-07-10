use crate::game::plugins::WorldPlugin;
use bevy::app::App;
use bevy::prelude::*;

mod game;
mod pcg;

fn main() {
    App::new()
        // .add_plugins(
        //     MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        //         1.0 / 30.0,
        //     ))),
        // )
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .run();
}
