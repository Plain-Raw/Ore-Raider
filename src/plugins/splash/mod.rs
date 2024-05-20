use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweeningPlugin};
use bevy_tweening::lens::TransformScaleLens;
use crate::gamestate::GameState;
use crate::plugins::colors;
use crate::plugins::menu::despawn_screen;

const INIT_TRANSITION_DONE: u64 = 1;


// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub fn splash_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        .add_plugins(TweeningPlugin)
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Px(1920.0),
            height: Val::Px(1500.0),
            ..default()
        },
        background_color: colors::BACKGROUND_COLOR.into(),
        ..default()
    }));

    let icon = asset_server.load("branding/logo-color.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {

            let tween_scale = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_millis(2500),
                TransformScaleLens {
                    start: Vec3::splat(0.01),
                    end: Vec3::ONE,
                },
            )
                .with_completed_event(INIT_TRANSITION_DONE);
;
            let animator =  Animator::new(tween_scale);

            parent.spawn((ImageBundle {
                style: Style {
                    width: Val::Px(1920.0),
                    height: Val::Px(1500.0),
                    ..default()
                },

                image: UiImage::new(icon),
                ..default()
            },  animator));
        });
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/drop.ogg"),
        ..default()
    });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(4.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}


