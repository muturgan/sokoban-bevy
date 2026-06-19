use bevy::ecs::{entity::Entity, resource::Resource};

use crate::embedded::LevelData;

// Ресурс для отслеживания состояния победы
#[derive(Resource, Default)]
pub struct GameState {
    pub won: bool,
    pub current_level: usize,
    pub game_complete: bool,
}

// Ресурс для хранения сущности уровня
#[derive(Resource)]
pub struct LevelEntity(pub Entity);

// Ресурс для хранения сущности UI победы
#[derive(Resource, Default)]
pub struct WinUIEntity(pub Option<Entity>);

// Ресурс для хранения сущности кнопки рестарта
#[derive(Resource, Default)]
pub struct RestartButtonEntity(pub Option<Entity>);

// Ресурс для хранения загруженных уровней
#[derive(Resource, Default)]
pub struct Levels {
    pub data: &'static [LevelData],
}
