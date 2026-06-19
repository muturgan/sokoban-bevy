# Bevy 0.19 ECS Patterns

Принципы и паттерны для работы с Bevy ECS версии 0.19, выявленные в процессе разработки Sokoban.

## Проблема B0001: Конфликт запросов (Query Conflict)

### Суть проблемы

Ошибка `error[B0001]` возникает, когда система пытается получить доступ к одному и тому же компоненту через несколько параметров запроса способом, который Bevy не может гарантировать как безопасный.

**Типичный сценарий:**
```rust
// ❌ НЕ РАБОТАЕТ — конфликт!
fn player_input(
    mut player_query: Query<&mut Transform, With<Player>>,      // мутабельный доступ
    box_query: Query<(Entity, &Transform), With<BoxMarker>>,    // доступ к Transform
    wall_query: Query<&Transform, With<Wall>>,                  // доступ к Transform
) { }
```

Даже если `With<Player>`, `With<BoxMarker>`, и `With<Wall>` логически не пересекаются, **Bevy требует явного указания `Without<T>`** для доказательства непересекаемости на уровне типов.

### Решение 1: Явные `Without` маркеры

Добавьте `Without<T>` к каждому query, чтобы явно указать, какие сущности исключаются:

```rust
// ✅ Работает — явные ограничения
fn player_input(
    mut player_query: Query<&mut Transform, (With<Player>, Without<BoxMarker>, Without<Wall>)>,
    mut box_query: Query<(Entity, &mut Transform), (With<BoxMarker>, Without<Player>, Without<Wall>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>, Without<BoxMarker>)>,
) { }
```

**Недостаток:** Многословно, сложно поддерживать при росте числа типов сущностей.

### Решение 2: Единый Query с `Option<&T>` (рекомендуется)

Используйте один query с опциональными компонентами для сбора данных, затем применяйте изменения через `get_many_mut`:

```rust
// ✅ Работает — один query, нет конфликта
fn player_input(
    mut query: Query<(
        Entity,
        &mut Transform,
        Option<&Player>,
        Option<&BoxMarker>,
    ), Without<Wall>>,  // Важно: исключает стены из query
    wall_query: Query<&Transform, With<Wall>>,
) {
    // Шаг 1: Собираем данные (immutable итерация)
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

    // Шаг 2: Вычисляем новую позицию
    let Some(new_pos) = compute_new_position(player_pos, ...) else { return };

    // Шаг 3: Применяем изменения через get_many_mut
    if let Some((box_entity, _)) = box_at_new_pos {
        let player_and_box = query.get_many_mut([player_entity, box_entity]);
        if let Ok([mut player_t, mut box_t]) = player_and_box {
            box_t.1.translation = box_new_pos;
            player_t.1.translation = new_pos;
        }
    }
}
```

**Преимущества:**
- Один query = нет конфликта
- Явная логика: сбор данных → вычисления → применение
- Легко расширять новыми типами сущностей

### Решение 3: `ParamSet` (для сложных случаев)

`ParamSet` позволяет разделить взаимоисключающие запросы, но требует осторожности с заимствованиями:

```rust
// ⚠️ Сложно в использовании — временные значения
fn player_input(
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<(Entity, &Transform), (With<BoxMarker>, Without<Player>)>,
        Query<&Transform, (With<Wall>, Without<Player>, Without<BoxMarker>)>,
    )>,
) {
    // Проблема: param_set.p0() создаёт временное значение
    let mut player_transform = param_set.p0().single_mut()?; // ❌ borrow checker

    // Решение: явное связывание
    let mut p0 = param_set.p0();
    let mut player_transform = p0.single_mut()?; // ✅
}
```

**Недостатки:**
- Сложный borrow checker
- Нельзя одновременно использовать `p0()` и `p1()` в одной области видимости
- Требует явного управления временем жизни заиммований

## Иерархия сущностей и компоненты родителя

### Проблема B0004

При создании иерархии с дочерними сущностями, родитель **должен** иметь `Transform` и `Visibility`, иначе будут предупреждения:

```rust
// ❌ Вызывает warning[B0004]
let level_root = commands.spawn((LevelRoot, Name::new("LevelRoot")))
    .with_children(|parent| {
        parent.spawn((Sprite::default(), Transform::default(), Wall));
    });

// ✅ Правильно — родитель с необходимыми компонентами
let level_root = commands.spawn((
    LevelRoot,
    Name::new("LevelRoot"),
    Transform::default(),      // Обязательно для иерархии
    Visibility::default(),     // Обязательно для иерархии
))
.with_children(|parent| {
    parent.spawn((Sprite::default(), Transform::default(), Wall));
});
```

## Чеклист для избежания B0001

1. **Проверьте все query в системе** — нет ли нескольких обращений к одному компоненту?
2. **Добавьте `Without<T>`** к каждому query, если они должны быть непересекающимися
3. **Рассмотрите единый query** с `Option<&T>` для простых случаев
4. **Используйте `get_many_mut`** для мутации нескольких сущностей одновременно
5. **Избегайте `ParamSet`** если можно обойтись более простым подходом

## Специфика Bevy 0.19

- **Строгая проверка заиммований** — Bevy 0.19 требует явных `Without<T>` даже для логически непересекающихся query
- **`get_many_mut`** — предпочтительный способ мутации нескольких сущностей в одном query
- **Иерархия** — родительские сущности должны иметь `Transform` и `Visibility`

---

*Документ создан 2026-06-18 после исправления ошибки B0001 в Sokoban*
