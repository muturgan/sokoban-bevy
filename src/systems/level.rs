use bevy::prelude::*;

use crate::{
    components::{BoxMarker, Floor, LevelRoot, Player, Target, Wall},
    constants::TILE_SIZE,
    embedded::Tile,
    resources::LevelEntity,
};

pub fn load_level(
    commands: &mut Commands,
    level_index: usize,
    levels: &[crate::embedded::LevelData],
) {
    let level_data = &levels[level_index % levels.len()];
    let width = level_data.width;
    let height = level_data.height;
    let tiles = level_data.tiles;

    // Создаём корневую сущность уровня с дочерними
    let level_root = commands
        .spawn((
            LevelRoot,
            Name::new("LevelRoot"),
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (y, row) in (0..height).enumerate() {
                for (x, col) in (0..width).enumerate() {
                    let idx = row * width + col;
                    let tile = tiles[idx];

                    let pos = Vec2::new(
                        (x as f32 - width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0,
                        -(y as f32 - height as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0,
                    );

                    match tile {
                        Tile::Wall => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.3, 0.3, 0.35),
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 0.0),
                                Wall,
                                Name::new("Wall"),
                            ));
                        }
                        Tile::Player => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.2, 0.6, 0.9),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 1.0),
                                Player,
                                Name::new("Player"),
                            ));
                        }
                        Tile::Crate => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.8, 0.5, 0.2),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.85)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 0.5),
                                BoxMarker,
                                Name::new("Box"),
                            ));
                        }
                        Tile::Target => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.3, 0.7, 0.3),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.5)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, -0.1),
                                Target,
                                Name::new("Target"),
                            ));
                        }
                        Tile::Empty => {
                            // Пол - видимая плитка
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.15, 0.15, 0.18),
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, -0.2),
                                Floor,
                                Name::new("Floor"),
                            ));
                        }
                    }
                }
            }
        })
        .id();

    // Сохраняем сущность уровня в ресурсе
    commands.insert_resource(LevelEntity(level_root));
}

/// Загружает уровень напрямую через World (без Commands)
pub fn load_level_direct(
    world: &mut World,
    level_index: usize,
    levels: &[crate::embedded::LevelData],
) {
    let level_data = &levels[level_index % levels.len()];
    let width = level_data.width as i32;
    let height = level_data.height as i32;
    let tiles = level_data.tiles;

    // Создаём корневую сущность уровня с дочерними
    let level_root = world
        .spawn((
            LevelRoot,
            Name::new("LevelRoot"),
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (y, row) in (0..height).enumerate() {
                for (x, col) in (0..width).enumerate() {
                    let idx = row * width + col;
                    let tile = tiles[idx as usize];

                    let pos = Vec2::new(
                        (x as f32 - width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0,
                        -(y as f32 - height as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0,
                    );

                    match tile {
                        Tile::Wall => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.3, 0.3, 0.35),
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 0.0),
                                Wall,
                                Name::new("Wall"),
                            ));
                        }
                        Tile::Player => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.2, 0.6, 0.9),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 1.0),
                                Player,
                                Name::new("Player"),
                            ));
                        }
                        Tile::Crate => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.8, 0.5, 0.2),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.85)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, 0.5),
                                BoxMarker,
                                Name::new("Box"),
                            ));
                        }
                        Tile::Target => {
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.3, 0.7, 0.3),
                                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.5)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, -0.1),
                                Target,
                                Name::new("Target"),
                            ));
                        }
                        Tile::Empty => {
                            // Пол - видимая плитка
                            parent.spawn((
                                Sprite {
                                    color: Color::srgb(0.15, 0.15, 0.18),
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                Transform::from_xyz(pos.x, pos.y, -0.2),
                                Floor,
                                Name::new("Floor"),
                            ));
                        }
                    }
                }
            }
        })
        .id();

    // Сохраняем сущность уровня в ресурсе
    world.insert_resource(LevelEntity(level_root));
}
