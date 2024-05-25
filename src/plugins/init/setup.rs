use bevy::prelude::{Component, Resource};

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct MenuMusicVolume(pub u32);

impl MenuMusicVolume {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GameMusicVolume(pub u32);

impl GameMusicVolume {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}
