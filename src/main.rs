mod test_system;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Update, test_system::hello_world)
        .run();
}

