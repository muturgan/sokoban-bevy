# Level Transitions

Реализация переходов между уровнями в Sokoban.

## Проблема

При переходе на следующий уровень через кнопку "Следующий уровень" персонаж переставал реагировать на управление до нажатия кнопки "Рестарт".

## Причины

1. **`game_state.won` не сбрасывался** — в `player_input` есть проверка `if game_state.won { return; }`, которая блокирует управление при победе
2. **UI победы не удалялся полностью** — использовался только корневой Entity, но не все дочерние элементы
3. **Уровень не перезагружался корректно** — состояние `Restarting` переключалось, но логика загрузки не выполнялась

## Решение

### 1. Отдельный маркер для контейнера UI победы

Добавлен компонент `WinUIContainer` для корневого элемента UI победы:

```rust
// components.rs
#[derive(Component)]
pub struct WinUIContainer;
```

Это позволяет удалить весь UI целиком (контейнер + дочерние элементы) через `Query<Entity, With<WinUIContainer>>`.

### 2. Сброс `game_state.won` в `restart_level`

Функция `restart_level` теперь явно сбрасывает флаг победы:

```rust
// systems/restart.rs
pub fn restart_level(world: &mut World) {
    // Сбрасываем состояние победы
    let mut game_state_mut = world.get_resource_mut::<GameState>().unwrap();
    game_state_mut.won = false;
    
    // ... удаление UI и уровня ...
    
    load_level_direct(world, current_level, &levels);
    next_mode.set(GameMode::Playing);
}
```

### 3. Прямая загрузка уровня через `World`

Используется `load_level_direct` вместо `Commands` для немедленного применения изменений:

```rust
// systems/level.rs
pub fn load_level_direct(world: &mut World, level_index: usize, levels: &[Vec<String>]) {
    let level_root = world.spawn(...).id();
    world.insert_resource(LevelEntity(level_root));
}
```

## Архитектура

### Состояния игры (GameMode)
- `Loading` — загрузка уровней из файлов
- `Playing` — игровой процесс, обработка ввода игрока
- `Restarting` — перезагрузка уровня (удаление старого, загрузка нового)

### Завершение игры

После прохождения последнего уровня показывается экран завершения игры вместо кнопки "Следующий уровень":

```rust
// resources.rs
pub struct GameState {
    pub won: bool,
    pub current_level: usize,
    pub game_complete: bool,  // Флаг завершения всей игры
}

// components.rs
#[derive(Component)]
pub struct GameCompleteUI;  // Маркер UI завершения игры
```

**Логика:**
1. `check_win` проверяет, был ли это последний уровень
2. Если да — спавнит `GameCompleteUI` (зелёная рамка, текст "Игра пройдена!")
3. Кнопка "Следующий уровень" не реагирует на нажатия на последнем уровне

### Поток перехода на следующий уровень
1. `next_level_button` (в состоянии `Playing`) → обновляет `current_level`, устанавливает `GameMode::Restarting`
2. `restart_level` (в состоянии `Restarting`) → сбрасывает `won`, удаляет UI и уровень, загружает новый, переключает на `Playing`

### Ключевые файлы
| Файл | Описание |
|------|----------|
| `src/components.rs` | `WinUIContainer`, `GameCompleteUI` — маркеры UI |
| `src/resources.rs` | `GameState` с полем `game_complete` |
| `src/systems/restart.rs` | `restart_level` — сброс состояния, загрузка уровня |
| `src/systems/level.rs` | `load_level_direct` — прямая загрузка через `World` |
| `src/systems/ui.rs` | `next_level_button` — блокируется на последнем уровне |
| `src/systems/win.rs` | `check_win`, `spawn_game_complete_ui` — проверка победы и завершение игры |
| `src/systems/player.rs` | `player_input` — проверка `game_state.won` |
| `src/embedded.rs` | `Tile`, `LevelData` — встроенные данные уровней |
| `build.rs` | Генерация констант уровней при компиляции |

## Почему это важно

- **`game_state.won` должен сбрасываться ДО загрузки нового уровня** — иначе `player_input` будет возвращаться сразу при входе
- **Использовать `World` напрямую для загрузки уровня** — `Commands` может не примениться до переключения состояния
- **Отдельный маркер для контейнера UI** — упрощает удаление всего UI целиком без обхода дочерних элементов

---

*Документ создан 2026-06-18 после исправления бага с переходом между уровнями в Sokoban*
