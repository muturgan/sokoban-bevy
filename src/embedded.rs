/// Встроенные ресурсы игры
/// Шрифты и уровни встраиваются прямо в бинарный файл при компиляции
/// Данные читаются из файлов assets/ во время сборки через build.rs
use bevy::prelude::*;

/// Тип клетки уровня
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Player = 2,
    Crate = 3,
    Target = 4,
}

/// Данные уровня
#[derive(Debug, Clone, Copy)]
pub struct LevelData {
    pub width: usize,
    pub height: usize,
    pub tiles: &'static [Tile],
}

// Включаем сгенерированный build.rs файл с данными
// env!("OUT_DIR") уже содержит правильный путь для платформы
include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

/// Ресурс для хранения Handle шрифта
#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

impl GameFont {
    pub fn get_source(&self) -> FontSource {
        self.0.clone().into()
    }
}
