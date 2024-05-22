use crate::gamestate::GameState;
use crate::plugins::init::setup::{DisplayQuality, Volume};
use crate::plugins::menu::menu_plugin;
use crate::plugins::splash::splash_plugin;
use bevy::app::{App, Plugin};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

pub mod setup;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::ERROR,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                    update_subscriber: None,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "OreRaider".into(),
                        name: Some("bevy.app".into()),
                        resolution: (1900., 1000.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(DisplayQuality::High)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash_plugin, menu_plugin));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
