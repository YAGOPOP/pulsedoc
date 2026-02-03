use dirs_next::document_dir;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{
    env::current_exe,
    fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    save_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let sd = match document_dir() {
            Some(v) => v.join("pulsedoc-output"),
            None => {
                eprintln!("Не удалось найти пользовательский каталог документов.");
                let failure_dir = get_exe_dir().join("pulsedoc-output");
                eprintln!("документы будут сохранены по пути {:?}", &failure_dir);
                failure_dir
            }
        };
        fs::create_dir(&sd).unwrap_or_else(|_| {});
        Self { save_dir: sd }
    }
}

impl Settings {
    pub fn get_save_dir(&self) -> PathBuf {
        self.save_dir.to_owned()
    }
}

pub fn get_exe_dir() -> PathBuf {
    match current_exe() {
        Ok(exe) => match exe.parent() {
            Some(dir) => dir.to_path_buf(),
            None => {
                eprintln!("Ошибка: некорректное расположение исполняемого файла");
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Ошибка: путь исполняемого файла не найден. ({})", e);
            process::exit(1);
        }
    }
}

pub fn load_settings() -> Settings {
    let path = get_exe_dir().join("settings").join("settings.json");
    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str::<Settings>(&text) {
            let _ = fs::create_dir_all(&settings.save_dir);
            println!(
                "Загружены настройки, rаталог сохранения: {:?}",
                &settings.save_dir
            );
            return settings;
        }
    }
    let save_dir = choose_save_dir_or_default();
    let settings = Settings { save_dir };
    save_settings(&path, &settings);
    settings
}

fn choose_save_dir_or_default() -> PathBuf {
    match FileDialog::new()
        .set_title("Выберите каталог для сохранения")
        .pick_folder()
    {
        Some(p) => {
            println!("Выбран каталог сохранения: {:?}", &p);
            p
        }
        None => {
            let p = Settings::default().save_dir;
            println!("действие отменено, выбран стандартный каталог: {:?}", &p);
            p
        }
    }
}

fn save_settings(path: &Path, settings: &Settings) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let text = serde_json::to_string_pretty(settings).unwrap();
    fs::write(path, text).unwrap();
}
