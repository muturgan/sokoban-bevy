use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};

use crate::{
    components::{LoadingUI, WinUI},
    resources::{CrateImage, Levels, PlayerImage},
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
pub fn next_level_button_hover(
    mut q: Query<
        (&Hovered, &mut BackgroundColor, &Children),
        (Changed<Hovered>, With<Button>, With<WinUI>),
    >,
    mut text_query: Query<&mut TextColor, With<WinUI>>,
) {
    for (hovered, mut color, children) in &mut q {
        let (bg, text) = if hovered.get() {
            (Color::srgb(0.2, 0.4, 0.7), Color::srgb(0.9, 0.9, 0.9))
        } else {
            (Color::srgb(0.1, 0.3, 0.5), Color::WHITE)
        };
        *color = BackgroundColor(bg);
        for child in children {
            if let Ok(mut text_color) = text_query.get_mut(*child) {
                *text_color = TextColor(text);
            }
        }
    }
}
