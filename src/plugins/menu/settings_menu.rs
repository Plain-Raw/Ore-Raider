use bevy::hierarchy::BuildChildren;
use bevy::prelude::{AlignItems, ButtonBundle, Commands, default, FlexDirection, JustifyContent, NodeBundle, Res, Style, TextBundle, TextStyle, UiRect, Val};
use crate::plugins::colors;
use crate::plugins::colors::{NORMAL_BUTTON, TEXT_COLOR};
use crate::plugins::menu::{MenuButtonAction, OnSettingsMenuScreen};

pub fn settings_menu_setup(mut commands: Commands) {
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
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vh(100.0),
                        height: Val::Vh(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

