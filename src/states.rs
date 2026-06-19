use bevy::state::state::States;

// Состояния игры
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameMode {
    #[default]
    Loading,
    Playing,
    Restarting,
}
