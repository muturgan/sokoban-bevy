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
        for line in &lines {
            for ch in line.chars() {
                match ch {
                    'B' => boxes += 1,
                    'T' => targets += 1,
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
