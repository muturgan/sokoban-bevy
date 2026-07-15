use bevy::ecs::component::Component;

// === Компоненты ===

#[derive(Component)]
pub struct Player;

// Направление взгляда игрока
#[derive(Component, Default, PartialEq, Clone, Copy)]
pub enum PlayerDirection {
    Left,
    #[default]
    Right,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct BoxMarker;

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Floor;

// Маркер корневой сущности уровня
#[derive(Component)]
pub struct LevelRoot;

// Маркер UI победы
#[derive(Component)]
pub struct WinUI;

// Маркер корневого контейнера UI победы
#[derive(Component)]
pub struct WinUIContainer;

// Маркер UI завершения игры
#[derive(Component)]
pub struct GameCompleteUI;

// Маркер UI рестарта
#[derive(Component)]
pub struct RestartButtonUI;

// Маркер UI загрузки
#[derive(Component)]
pub struct LoadingUI;
