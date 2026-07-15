# Notes for AI Agents

## Project Overview

**Sokoban** — puzzle game built with **Bevy 0.19.0** (Rust).

Features:
- Multiple levels with target positions for boxes
- Victory UI with "Next Level" button
- Game completion screen after finishing all levels
- Custom font: FiraSans Bold (TTF)

## Key Documentation

### 📖 [docs/BEVY_ECS_PATTERNS.md](docs/BEVY_ECS_PATTERNS.md)

**Read this first when working on game logic.** Contains:

1. **Bevy 0.19 ECS Patterns** — avoiding query conflicts (B0001), using `Without<T>`, `get_many_mut`
2. **Entity Hierarchy** — parent components (`Transform`, `Visibility`)

### 📖 [docs/LEVEL_TRANSITIONS.md](docs/LEVEL_TRANSITIONS.md)

**Read when working on level transitions.** Contains:

1. **Critical requirements** — reset `game_state.won`, use `WinUIContainer`, load via `World`
2. **Architecture** — game states, transition flow, key files
3. **Common pitfalls** — why player can't move after next level
4. **Game completion** — end screen after finishing all levels

### 📖 [docs/LEVEL_FORMAT.md](docs/LEVEL_FORMAT.md)

**Read when creating or editing levels.** Contains:

1. **Tile symbols** — `#`, `P`, `B`, `T`, ` ` (space)
2. **Level rules** — required elements, size limits, wall placement
3. **Examples** — simple to complex levels
4. **Debugging** — compile-time validation (boxes == targets), unknown symbols

## Architecture

### Game States
- `Loading` → `Playing` → `Restarting` → `Playing`

### Key Files

**Root (`src/`):**
| File | Description |
|------|-------------|
| `main.rs` | App setup, states, system registration |
| `components.rs` | Component markers (`Player`, `Wall`, `BoxMarker`, `WinUIContainer`, `GameCompleteUI`, etc.) |
| `constants.rs` | Game constants (`TILE_SIZE`) |
| `embedded.rs` | `Tile` enum, `LevelData` struct, embedded assets |
| `levels.rs` | Level loading from embedded data |
| `resources.rs` | Game resources (`GameState`, `Levels`, `LevelEntity`, `WinUIEntity`, etc.) |
| `states.rs` | Game state enum (`GameMode`) |

**Build:**
| File | Description |
|------|-------------|
| `build.rs` | Reads level `.txt` files and font at compile time, generates constants |

**Systems (`src/systems/`):**
| File | Description |
|------|-------------|
| `player.rs` | Player input handling |
| `level.rs` | Level loading from `LevelData` (`load_level`, `load_level_direct`) |
| `restart.rs` | Level transition logic (`restart_level`) |
| `ui.rs` | UI buttons (`next_level_button`, `check_loaded_levels`) |
| `win.rs` | Win condition check, victory UI spawn, game completion UI |

## Common Pitfalls

⚠️ **Level transition bug:** If player can't move after next level, check:
1. `game_state.won` is reset in `restart_level()`
2. UI victory container is fully despawned
3. Level is loaded via `World` (not `Commands`)

See [docs/LEVEL_TRANSITIONS.md](docs/LEVEL_TRANSITIONS.md) for details.

## Development Workflow

### Before Reporting Completion

**Always run `cargo clippy` after making code changes** before telling the user that changes are complete:

```bash
cargo clippy
```

If there are errors:
1. Fix all compilation errors
2. Re-run `cargo clippy` until it passes
3. Run `cargo +nightly fmt` to apply a code style
4. Only then report completion to the user

This ensures you don't report "done" for broken code.
