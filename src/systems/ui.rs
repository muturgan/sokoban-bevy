use bevy::prelude::*;

use crate::{
    components::{LoadingUI, WinUI},
    resources::{CrateImage, GameState, Levels, PlayerImage},
    states::GameMode,
    systems::level::load_level,
};

pub fn check_loaded_levels(
    mut commands: Commands,
    levels: Option<Res<Levels>>,
    loading_query: Query<Entity, With<LoadingUI>>,
    mut next_mode: ResMut<NextState<GameMode>>,
    player_image: Option<Res<PlayerImage>>,
    crate_image: Option<Res<CrateImage>>,
) {
    if let Some(levels_res) = levels
        && let Some(player_img) = player_image
        && let Some(crate_img) = crate_image
    {
        // Удаляем UI загрузки
        for entity in loading_query.iter() {
            commands.entity(entity).despawn();
        }
        // Переключаем в режим игры
        next_mode.set(GameMode::Playing);
        // Загружаем первый уровень
        load_level(
            &mut commands,
            0,
            levels_res.data,
            &player_img.0,
            &crate_img.0,
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn next_level_button(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, With<WinUI>),
    >,
    mut text_query: Query<&mut TextColor, With<WinUI>>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mut game_state: ResMut<GameState>,
    levels: Res<Levels>,
) {
    for (interaction, mut color, children) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                // Проверяем, не последний ли это уровень
                let is_last_level = game_state.current_level >= levels.data.len() - 1;
                if is_last_level {
                    // Игра завершена, не переходим на следующий уровень
                    return;
                }

                // Переход на следующий уровень
                game_state.current_level = (game_state.current_level + 1) % levels.data.len();
                game_state.won = false;
                // Переключаем в режим рестарта для загрузки нового уровня
                next_mode.set(GameMode::Restarting);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.2, 0.4, 0.7));
                for child in children {
                    if let Ok(mut text_color) = text_query.get_mut(*child) {
                        *text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
                    }
                }
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.1, 0.3, 0.5));
                for child in children {
                    if let Ok(mut text_color) = text_query.get_mut(*child) {
                        *text_color = TextColor(Color::WHITE);
                    }
                }
            }
        }
    }
}
