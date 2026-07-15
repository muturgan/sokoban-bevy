use bevy::prelude::*;

use crate::{
    components::{BoxMarker, Player, PlayerDirection, Wall},
    constants::TILE_SIZE,
    resources::GameState,
};

#[allow(clippy::type_complexity)]
pub fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Transform, Option<&Player>, Option<&BoxMarker>), Without<Wall>>,
    wall_query: Query<&Transform, With<Wall>>,
    game_state: Res<GameState>,
) {
    if game_state.won {
        return;
    }

    // Собираем данные о позиции игрока и ящиков
    let mut player_entity = None;
    let mut player_pos = None;
    let mut boxes = Vec::new();

    for (e, t, player, box_m) in query.iter() {
        if player.is_some() {
            player_entity = Some(e);
            player_pos = Some(t.translation);
        }
        if box_m.is_some() {
            boxes.push((e, t.translation));
        }
    }

    let Some((player_entity, player_pos)) = player_entity.zip(player_pos) else {
        return;
    };

    let direction = if keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyW)
    {
        Some(Vec2::new(0.0, TILE_SIZE))
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        Some(Vec2::new(0.0, -TILE_SIZE))
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        Some(Vec2::new(-TILE_SIZE, 0.0))
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        Some(Vec2::new(TILE_SIZE, 0.0))
    } else {
        None
    };

    if let Some(dir) = direction {
        let new_pos = player_pos + dir.extend(0.0);

        // Проверяем, есть ли стена в новой позиции
        let collides_with_wall = wall_query.iter().any(|wall_transform| {
            let dx = (wall_transform.translation.x - new_pos.x).abs();
            let dy = (wall_transform.translation.y - new_pos.y).abs();
            dx < TILE_SIZE * 0.5 && dy < TILE_SIZE * 0.5
        });

        if collides_with_wall {
            return;
        }

        // Проверяем, есть ли ящик в новой позиции
        let box_at_new_pos = boxes.iter().find(|(_, pos)| {
            let dx = (pos.x - new_pos.x).abs();
            let dy = (pos.y - new_pos.y).abs();
            dx < TILE_SIZE * 0.5 && dy < TILE_SIZE * 0.5
        });

        if let Some((box_entity, _)) = box_at_new_pos {
            // Пытаемся толкнуть ящик
            let box_new_pos = new_pos + dir.extend(0.0);

            // Проверяем, можно ли толкнуть ящик (нет стены и другого ящика)
            let box_collides_with_wall = wall_query.iter().any(|wall_transform| {
                let dx = (wall_transform.translation.x - box_new_pos.x).abs();
                let dy = (wall_transform.translation.y - box_new_pos.y).abs();
                dx < TILE_SIZE * 0.5 && dy < TILE_SIZE * 0.5
            });

            let box_collides_with_box = boxes.iter().any(|(e, pos)| {
                *e != *box_entity && {
                    let dx = (pos.x - box_new_pos.x).abs();
                    let dy = (pos.y - box_new_pos.y).abs();
                    dx < TILE_SIZE * 0.5 && dy < TILE_SIZE * 0.5
                }
            });

            if !box_collides_with_wall && !box_collides_with_box {
                // Двигаем ящик и игрока через get_many_mut
                let player_and_box = query.get_many_mut([player_entity, *box_entity]);
                if let Ok([mut player_t, mut box_t]) = player_and_box {
                    box_t.1.translation = box_new_pos;
                    player_t.1.translation = new_pos;
                }
            }
        } else {
            // Просто двигаем игрока
            if let Ok((_, mut t, _, _)) = query.get_mut(player_entity) {
                t.translation = new_pos;
            }
        }
    }
}

/// Обновляет направление взгляда игрока и flip_x спрайта
pub fn update_player_direction(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerDirection, &mut Sprite), With<Player>>,
) {
    for (mut direction, mut sprite) in query.iter_mut() {
        let new_direction =
            if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
                Some(PlayerDirection::Right)
            } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
                Some(PlayerDirection::Left)
            } else {
                None
            };

        if let Some(dir) = new_direction
            && dir != *direction
        {
            *direction = dir;
            // flip_x: true = смотрит вправо, false = смотрит влево
            sprite.flip_x = dir == PlayerDirection::Left;
        }
    }
}
