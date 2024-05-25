use crate::plugins::init::setup::GameMusicVolume;
use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings};
use bevy::prelude::{default, Commands, Component, DetectChanges, Query, Res, With};
use std::ops::Div;

pub fn game_music_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/nordic-game-music.ogg"),
            settings: {
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    paused: true,
                    speed: 1.0,
                    ..default()
                }
            },
        },
        GameMusic,
    ));
}

#[derive(Component)]
pub struct GameMusic;

pub fn game_music_pause(music_controller: Query<&AudioSink, With<GameMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.stop()
    }
}

pub fn game_music_unpause(music_controller: Query<&AudioSink, With<GameMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.play()
    }
}

pub fn on_game_volume_change(
    my_res: Option<Res<GameMusicVolume>>,
    music_controller: Query<&AudioSink, With<GameMusic>>,
) {
    if let Some(my_res) = my_res {
        // the resource exists
        if my_res.is_changed() {
            if let Ok(sink) = music_controller.get_single() {
                let f = my_res.as_f32();
                if f == 0. {
                    sink.set_volume(0.);
                } else {
                    sink.set_volume(f.div(10.));
                }
            }
        }
    }
}
