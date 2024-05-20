use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{AlignItems, ButtonBundle, Commands, default, FlexDirection, JustifyContent, NodeBundle, Res, Style, TextBundle, TextStyle, UiRect, Val};
use crate::plugins::colors;
use crate::plugins::colors::{BACKGROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR};
use crate::plugins::init::setup::DisplayQuality;
use crate::plugins::menu::{MenuButtonAction, OnDisplaySettingsMenuScreen, SelectedOption};





pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
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
            OnDisplaySettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color:  colors::BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Display a label for the current setting
                            display_settings_button(display_quality, &button_style, &button_text_style, parent);
                        });
                    // Display the back button to return to the settings screen
                    back_button_to_settings(button_style, button_text_style, parent);
                });
        });
}

fn display_settings_button(display_quality: Res<DisplayQuality>, button_style: &Style, button_text_style: &TextStyle, parent: &mut ChildBuilder) {
    parent.spawn(TextBundle::from_section(
        "Display Quality",
        button_text_style.clone(),
    ));
    // Display a button for each possible value
    for quality_setting in [
        DisplayQuality::Low,
        DisplayQuality::Medium,
        DisplayQuality::High,
    ] {
        let mut entity = parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    ..button_style.clone()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            quality_setting,
        ));
        entity.with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("{quality_setting:?}"),
                button_text_style.clone(),
            ));
        });
        if *display_quality == quality_setting {
            entity.insert(SelectedOption);
        }
    }
}

fn back_button_to_settings(button_style: Style, button_text_style: TextStyle, parent: &mut ChildBuilder) {
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
}