use crate::plugins::colors;
use crate::plugins::colors::{NORMAL_BUTTON, TEXT_COLOR};
use crate::plugins::init::setup::{GameMusicVolume, MenuMusicVolume};
use crate::plugins::menu::{
    GameMusicSelectedOption, MenuButtonAction, OnSoundSettingsMenuScreen, SelectedOption,
};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    default, AlignItems, ButtonBundle, Commands, FlexDirection, JustifyContent, NodeBundle, Res,
    Style, TextBundle, TextStyle, UiRect, Val,
};

pub fn sound_settings_menu_setup(
    mut commands: Commands,
    menu_music_volume: Res<MenuMusicVolume>,
    game_music_volume: Res<GameMusicVolume>,
) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSoundSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: colors::BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Menu Music Volume",
                                button_text_style.clone(),
                            ));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    MenuMusicVolume(volume_setting),
                                ));
                                if *menu_music_volume == MenuMusicVolume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                            parent.spawn(TextBundle::from_section(
                                "Game Music Volume",
                                button_text_style.clone(),
                            ));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    GameMusicVolume(volume_setting),
                                ));
                                if *game_music_volume == GameMusicVolume(volume_setting) {
                                    entity.insert(GameMusicSelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}
