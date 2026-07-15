use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_assets.rs");
    let mut file = fs::File::create(&dest_path)?;

    // === Шрифт ===
    let font_path = Path::new("assets/fonts/FiraSans-Bold.ttf");
    println!("cargo:rerun-if-changed={}", font_path.display());

    let font_bytes = fs::read(font_path)?;
    writeln!(file, "pub const FONT_DATA: &[u8] = &{:?};", font_bytes)?;

    // === Уровни ===
    let levels_dir = Path::new("assets/levels");
    println!("cargo:rerun-if-changed={}", levels_dir.display());

    let mut entries: Vec<_> = fs::read_dir(levels_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "txt")
                .unwrap_or(false)
        })
        .collect();

    // Сортируем по имени файла
    entries.sort_by_key(|e| e.file_name());

    // Генерируем константы для каждого уровня
    for (idx, entry) in entries.iter().enumerate() {
        let content = fs::read_to_string(entry.path())?;
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        // Валидация: количество ящиков == количество целей
        let mut boxes = 0;
        let mut targets = 0;
        let mut players = 0;
        for line in &lines {
            for ch in line.chars() {
                match ch {
                    'B' => boxes += 1,
                    'T' => targets += 1,
                    'P' => players += 1,
                    _ => {}
                }
            }
        }
        if boxes != targets {
            panic!(
                "Уровень {:?}: количество ящиков ({boxes}) не равно количеству целей ({targets})",
                entry.file_name()
            );
        }
        if players == 0 {
            panic!("Уровень {:?}: отсутствует игрок (P)", entry.file_name());
        }
        if players > 1 {
            panic!(
                "Уровень {:?}: слишком много игроков ({players}), должен быть ровно 1",
                entry.file_name()
            );
        }

        // Валидация: периметр стен должен быть замкнут
        // Используем flood fill из точки за пределами уровня — если можем достичь игрока, периметр не замкнут
        let grid: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                let mut row: Vec<char> = l.chars().collect();
                // Дополняем строку до width пробелами
                while row.len() < width {
                    row.push(' ');
                }
                row
            })
            .collect();

        // Создаём расширенную сетку с границей из пробелов вокруг уровня
        let extended_height = height + 2;
        let extended_width = width + 2;
        let mut extended: Vec<Vec<char>> = vec![vec![' '; extended_width]; extended_height];

        // Копируем уровень в центр расширенной сетки
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                extended[y + 1][x + 1] = *ch;
            }
        }

        // Flood fill из (0, 0) — если достигаем игрока 'P', периметр не замкнут
        let mut visited = vec![vec![false; extended_width]; extended_height];
        let mut stack = vec![(0usize, 0usize)];
        let mut player_reachable = false;

        while let Some((x, y)) = stack.pop() {
            if x >= extended_width || y >= extended_height {
                continue;
            }
            if visited[y][x] {
                continue;
            }
            let ch = extended[y][x];
            if ch == '#' {
                continue; // Стена — не проходим
            }
            if ch == 'P' {
                player_reachable = true;
                break;
            }
            visited[y][x] = true;
            // Добавляем соседей (4 направления)
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < extended_width {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < extended_height {
                stack.push((x, y + 1));
            }
        }

        if player_reachable {
            panic!(
                "Уровень {:?}: периметр стен не замкнут — игрок может выйти за пределы уровня",
                entry.file_name()
            );
        }

        // Генерируем плоский массив тайлов
        write!(
            file,
            "const LEVEL_{idx}_DATA: [Tile; {}] = [",
            width * height
        )?;
        for line in &lines {
            for ch in line.chars() {
                let tile = match ch {
                    ' ' => "Tile::Empty",
                    '#' => "Tile::Wall",
                    'P' => "Tile::Player",
                    'B' => "Tile::Crate",
                    'T' => "Tile::Target",
                    _ => panic!("Неизвестный символ '{ch}' в уровне {:?}", entry.file_name()),
                };
                write!(file, "{tile}, ")?;
            }
            // Дополняем строку до width пустыми клетками
            for _ in line.len()..width {
                write!(file, "Tile::Empty, ")?;
            }
        }
        writeln!(file, "];")?;
    }

    // Генерируем массив уровней
    writeln!(file, "pub const LEVELS: &[LevelData] = &[")?;
    for (idx, entry) in entries.iter().enumerate() {
        let content = fs::read_to_string(entry.path())?;
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        writeln!(
            file,
            "    LevelData {{ width: {width}, height: {height}, tiles: &LEVEL_{idx}_DATA }},",
        )?;
    }
    writeln!(file, "];")?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
