# Sokoban — Project Structure

```
sokoban-bevy/
├── .editorconfig                           — Cross-editor coding style settings
├── .gitignore                              — Cargo/target build artifacts and IDE folder exclusions
├── .vscode/
│   └── settings.json                       — VS Code config
├── AGENTS.md                               — AI agent onboarding: project overview, architecture, key docs, pitfalls
├── Cargo.lock                              — Locked dependency versions
├── Cargo.toml                              — Rust project manifest
├── LICENSE                                 — Project license file
├── README.md                               — Brief project description
├── assets/
│   ├── fonts/
│   │   └── FiraSans-Bold.ttf               — Custom TTF font embedded at compile time via build.rs
│   ├── images/
│   │   ├── char.png                        — Player sprite (loaded by AssetServer at runtime)
│   │   └── crate.png                       — Box/crate sprite (loaded by AssetServer at runtime)
│   └── levels/
│       ├── level_01.txt                    — Level 1: tutorial-level puzzle layout
│       ├── level_02.txt                    — Level 2: harder puzzle layout
│       ├── level_03.txt                    — Level 3: harder puzzle layout
│       ├── level_04.txt                    — Level 4: harder puzzle layout
│       └── level_05.txt                    — Level 5: harder puzzle layout
├── build.rs                                — Compile-time asset embedder: reads .ttf + .txt, validates levels, generates embedded_assets.rs
├── docs/
│   ├── BEVY_ECS_PATTERNS.md                — Bevy 0.19 ECS query conflict (B0001) patterns and solutions
│   ├── LEVEL_FORMAT.md                     — Level file syntax, validation rules, examples
│   └── LEVEL_TRANSITIONS.md                — Level transition implementation: win-flag reset, UI cleanup, World-based loading
├── rustfmt.toml                            — Rust formatter config
├── src/
│   ├── main.rs                             — App entry point: Bevy App builder, plugins, systems registration, Startup setup
│   ├── components.rs                       — ECS component markers: Player, Wall, BoxMarker, Target, Floor, WinUI, LevelRoot, etc.
│   ├── constants.rs                        — Game constants: TILE_SIZE = 64.0
│   ├── embedded.rs                         — Tile/LevelData types, generated-asset include, GameFont resource wrapper
│   ├── levels.rs                           — Level loading function returning &'static [LevelData] from embedded data
│   ├── resources.rs                        — ECS resources: GameState, Levels, PlayerImage, CrateImage, LevelEntity, WinUIEntity, etc.
│   ├── states.rs                           — Game state enum (GameMode): Loading → Playing → Restarting
│   └── systems/
│       ├── mod.rs                          — Systems module re-exports all public functions
│       ├── level.rs                        — Level loading system: spawns tiles, player, crates, walls, targets as entity hierarchy
│       ├── player.rs                       — Player input handling: arrow/WASD movement, crate pushing, direction tracking
│       ├── restart.rs                      — Level restart/transition: despawns old level, spawns new, resets game state
│       ├── ui.rs                           — UI systems: loading screen removal, "Next Level" button handler, restart button
│       └── win.rs                          — Win condition: box-on-target check, victory UI spawn, game-complete end screen
```
