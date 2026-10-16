use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::{
    components::{GameCompleteUI, RestartButtonUI, WinUIContainer},
    embedded::GameFont,
    resources::{
        CrateImage, GameState, LevelEntity, Levels, PlayerImage, RestartButtonEntity, WinUIEntity,
    },
    states::GameMode,
    systems::load_level_direct,
};

pub fn spawn_restart_button(
    mut commands: Commands,
    font: Res<GameFont>,
    button_entity: Res<RestartButtonEntity>,
) {
    // Создаём кнопку только если её ещё нет
    if button_entity.0.is_some() {
        return;
    }

    let entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            Button,
            Hovered::default(),
            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            RestartButtonUI,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Рестарт"),
                TextFont {
                    font: font.get_source(),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                RestartButtonUI,
            ));
        })
        .observe(on_restart_button_activate)
        .id();

    commands.insert_resource(RestartButtonEntity(Some(entity)));
}

fn on_restart_button_activate(
    _trigger: On<Activate>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ui_entity: ResMut<WinUIEntity>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    // Сбрасываем состояние
    game_state.won = false;

    // Удаляем UI победы если есть
    if let Some(entity) = ui_entity.0.take() {
        commands.entity(entity).despawn();
    }

    // Переключаем в режим рестарта
    next_mode.set(GameMode::Restarting);
}

#[allow(clippy::type_complexity)]
pub fn restart_button_hover(
    mut q: Query<(&Hovered, &mut BackgroundColor), (Changed<Hovered>, With<RestartButtonUI>)>,
) {
    for (hovered, mut color) in &mut q {
        *color = BackgroundColor(if hovered.get() {
            Color::srgb(0.3, 0.3, 0.4)
        } else {
            Color::srgb(0.2, 0.2, 0.25)
        });
    }
}

pub fn restart_request(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mut game_state: ResMut<GameState>,
    levels: Res<Levels>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        if game_state.won {
            game_state.current_level = (game_state.current_level + 1) % levels.data.len();
            println!("Загрузка уровня {}", game_state.current_level + 1);
        }
        game_state.won = false;
        next_mode.set(GameMode::Restarting);
    }
}

pub fn restart_level(world: &mut World) {
    // Получаем текущий уровень
    let Some(game_state) = world.get_resource::<GameState>() else {
        return;
    };
    let current_level = game_state.current_level;
    let level_entity_opt = world.get_resource::<LevelEntity>().map(|r| r.0);
    let levels = world.get_resource::<Levels>().unwrap().data;
    let player_image = world.get_resource::<PlayerImage>().unwrap().0.clone();
    let crate_image = world.get_resource::<CrateImage>().unwrap().0.clone();

    // Сбрасываем состояние победы
    let mut game_state_mut = world.get_resource_mut::<GameState>().unwrap();
    game_state_mut.won = false;
    game_state_mut.game_complete = false;

    // Удаляем корневую сущность уровня
    if let Some(level_entity) = level_entity_opt {
        world.despawn(level_entity);
    }

    // Удаляем все сущности UI победы (контейнер и дочерние)
    let win_ui_entities: Vec<Entity> = world
        .query_filtered::<Entity, With<WinUIContainer>>()
        .iter(world)
        .collect();
    for entity in win_ui_entities {
        world.despawn(entity);
    }

    // Удаляем UI завершения игры
    let game_complete_entities: Vec<Entity> = world
        .query_filtered::<Entity, With<GameCompleteUI>>()
        .iter(world)
        .collect();
    for entity in game_complete_entities {
        world.despawn(entity);
    }

    // Сбрасываем ресурс UI entity
    let mut ui_entity = world.get_resource_mut::<WinUIEntity>().unwrap();
    ui_entity.0 = None;

    // Загружаем новый уровень напрямую через world.spawn
    load_level_direct(world, current_level, levels, &player_image, &crate_image);

    // Переключаем состояние обратно
    let mut next_mode = world.get_resource_mut::<NextState<GameMode>>().unwrap();
    next_mode.set(GameMode::Playing);
}
