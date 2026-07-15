use bevy::{prelude::*, window::WindowResolution};

mod components;
mod constants;
mod embedded;
mod levels;
mod resources;
mod states;
mod systems;

use components::*;
use embedded::{FONT_DATA, GameFont};
use levels::load_levels;
use resources::{CrateImage, GameState, Levels, PlayerImage, RestartButtonEntity, WinUIEntity};
use states::GameMode;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sokoban".to_string(),
                resolution: WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_resource::<WinUIEntity>()
        .init_resource::<RestartButtonEntity>()
        .init_state::<GameMode>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            check_loaded_levels.run_if(in_state(GameMode::Loading)),
        )
        .add_systems(
            Update,
            (
                player_input,
                check_win,
                spawn_restart_button,
                update_player_direction,
            )
                .chain()
                .run_if(in_state(GameMode::Playing)),
        )
        .add_systems(
            Update,
            (restart_level, restart_request).run_if(in_state(GameMode::Restarting)),
        )
        .add_systems(
            Update,
            (next_level_button, restart_button_handler).run_if(in_state(GameMode::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2d);

    // Загружаем шрифт из встроенных данных и сохраняем в ресурс
    let font_handle = asset_server.add(Font::from_bytes(FONT_DATA.to_vec()));
    commands.insert_resource(GameFont(font_handle.clone()));

    // Загружаем изображение игрока
    let player_image = asset_server.load("images/char.png");
    commands.insert_resource(PlayerImage(player_image));

    // Загружаем изображение ящика
    let crate_image = asset_server.load("images/crate.png");
    commands.insert_resource(CrateImage(crate_image));

    // Загружаем уровни из встроенных данных
    let levels_data = load_levels();
    commands.insert_resource(Levels { data: levels_data });

    // Спавним UI загрузки
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            LoadingUI,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Загрузка..."),
                TextFont {
                    font: font_handle.into(),
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                LoadingUI,
            ));
        });
}
