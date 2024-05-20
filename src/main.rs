mod plugins;
mod gamestate;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((plugins::init::InitPlugin, plugins::game::game_plugin))
        .run();
}

