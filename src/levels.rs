use crate::embedded::{LEVELS, LevelData};

/// Загружает все уровни из встроенных данных
/// Уровни встроены в бинарный файл при компиляции
pub const fn load_levels() -> &'static [LevelData] {
    LEVELS
}
