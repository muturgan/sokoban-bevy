use bevy::prelude::*;

use crate::{
    components::{BoxMarker, GameCompleteUI, Target, WinUI, WinUIContainer},
    embedded::GameFont,
    resources::{GameState, Levels, WinUIEntity},
};

pub fn check_win(
    box_query: Query<&Transform, With<BoxMarker>>,
    target_query: Query<&Transform, With<Target>>,
    mut game_state: ResMut<GameState>,
    mut ui_entity: ResMut<WinUIEntity>,
    levels: Res<Levels>,
    mut commands: Commands,
    font: Res<GameFont>,
) {
    // Проверяем, все ли ящики на целевых позициях
    let all_boxes_on_targets = box_query.iter().all(|box_transform| {
        target_query.iter().any(|target_transform| {
            (box_transform.translation.truncate() - target_transform.translation.truncate())
                .length()
                < 0.1
        })
    });

    if all_boxes_on_targets && !game_state.won {
        game_state.won = true;

        // Проверяем, был ли это последний уровень
        let is_last_level = game_state.current_level >= levels.data.len() - 1;

        if is_last_level {
            // Завершение игры
            game_state.game_complete = true;
            if ui_entity.0.is_none() {
                ui_entity.0 = Some(spawn_game_complete_ui(&mut commands, &font));
            }
        } else {
            // Победа на уровне
            if ui_entity.0.is_none() {
                ui_entity.0 = Some(spawn_win_ui(&mut commands, &font));
            }
        }
    }
}

pub fn spawn_win_ui(commands: &mut Commands, font: &GameFont) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(30.0),
                border: UiRect::all(Val::Px(4.0)),
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            BorderColor::all(Color::srgb(0.8, 0.7, 0.2)),
            WinUIContainer,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    WinUI,
                ))
                .with_children(|p| {
                    // Заголовок
                    p.spawn((
                        Text::new("Победа!"),
                        TextFont {
                            font: font.get_source(),
                            font_size: FontSize::Px(48.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.8, 0.3)),
                        WinUI,
                    ));

                    // Подзаголовок
                    p.spawn((
                        Text::new("Все ящики на местах!"),
                        TextFont {
                            font: font.get_source(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        WinUI,
                    ));

                    // Кнопка "Следующий уровень"
                    p.spawn((
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.3, 0.6, 0.9)),
                        BackgroundColor(Color::srgb(0.1, 0.3, 0.5)),
                        WinUI,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("→ Следующий уровень"),
                            TextFont {
                                font: font.get_source(),
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            WinUI,
                        ));
                    });
                });
        })
        .id()
}

pub fn spawn_game_complete_ui(commands: &mut Commands, font: &GameFont) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                margin: UiRect::axes(Val::Auto, Val::Auto),
                border: UiRect::all(Val::Px(4.0)),
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            BorderColor::all(Color::srgb(0.2, 0.7, 0.3)),
            GameCompleteUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    WinUI,
                ))
                .with_children(|p| {
                    // Заголовок
                    p.spawn((
                        Text::new("Игра пройдена!"),
                        TextFont {
                            font: font.get_source(),
                            font_size: FontSize::Px(42.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.3, 0.9, 0.4)),
                        WinUI,
                    ));

                    // Подзаголовок
                    p.spawn((
                        Text::new("Все уровни завершены!"),
                        TextFont {
                            font: font.get_source(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        WinUI,
                    ));
                });
        })
        .id()
}
