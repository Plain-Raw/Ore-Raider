use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings};
use bevy::prelude::{default, Commands, Component, Query, Res, With};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/piano-loop.ogg"),
            settings: {
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    speed: 1.0,
                    ..default()
                }
            },
        },
        MenuMusic,
    ));
}

#[derive(Component)]
pub struct MenuMusic;

pub fn pause(music_controller: Query<&AudioSink, With<MenuMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.stop()
    }
}
