#![allow(clippy::type_complexity)]

mod gamestate;
mod plugins;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((plugins::init::InitPlugin, plugins::game::game_plugin))
        .run();
}
