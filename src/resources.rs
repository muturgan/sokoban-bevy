use bevy::{
    ecs::{entity::Entity, resource::Resource},
    prelude::{Handle, Image},
};

use crate::embedded::LevelData;

// Ресурс для хранения изображения игрока
#[derive(Resource)]
pub struct PlayerImage(pub Handle<Image>);

// Ресурс для хранения изображения ящика
#[derive(Resource)]
pub struct CrateImage(pub Handle<Image>);

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
