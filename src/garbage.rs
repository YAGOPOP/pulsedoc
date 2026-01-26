use chrono::{DateTime, Datelike, Local, NaiveDate};
use inquire::{InquireError, Select, Text};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{
    env::current_exe,
    // fmt,
    path::{Path, PathBuf},
};
use std::{fs, process};

#[derive(Debug, Serialize)]
pub struct EchoReport {
    pub name: String,
    pub birthday: String,
    pub department: String,
    pub cardnum: String,
    pub age: String,
    pub height: String,
    pub weight: String,
    pub pulse: String,
    pub aortic_sinus_diameter: String,
    pub body_surface_area: String,
    pub left_ventricle_diastolic_size: String,
    pub left_ventricle_systolic_size: String,
    pub septum_thickness: String,
    pub posterior_wall_thickness: String,
    pub left_ventricle_mass: String,
    pub left_ventricle_mass_index: String,
    pub relative_wall_thickness: String,
    pub stroke_volume: String,
    pub cardiac_index: String,
    pub cardiac_output: String,
    pub simpson_end_diastolic_volume: String,
    pub simpson_end_systolic_volume: String,
    pub ejection_fraction: String,
    pub ascending_aorta_diameter: String,
    pub left_atrium: String,
    pub left_atrium4: String,
    pub left_atrium_volume: String,
    pub left_atrium_index: String,
    pub right_atrium_s: String,
    pub right_atrium4: String,
    pub right_atrium_volume: String,
    pub right_ventricle: String,
    pub right_ventricle_baz: String,
    pub right_ventricle_medium_full: String,
    pub right_ventricle_wall_thickness_full: String,
    pub tapse_full: String,
    pub septum_thickness_baz_full: String,
    pub shutters_aortal: String,
    pub opening_amplitude: String,
    pub max_velocity: String,
    pub max_grad: String,
    pub mid_grad_full: String,
    pub s_doppler_full: String,
    pub s_planim_full: String,
    pub presh_time_full: String,
    pub vena_contracta_full: String,
    pub max_velocity_vt_full: String,
    pub max_grad_vt_full: String,
    pub shutters_mitral: String,
    pub peak_e: String,
    pub peak_a: String,
    pub peak_e_div_peak_a: String,
    pub tdi_vel: String,
    pub e_sept: String,
    pub e_lat: String,
    pub e_div_e_aps: String,
    pub max_velocity_mitral_valve_full: String,
    pub max_grad_mitral_valve_full: String,
    pub mid_grad_mitral_valve_full: String,
    pub calts_back_sash: String,
    pub posterior_leaflet_base_calcification: String,
    pub max_velocity_tricuspidal_regurgitation: String,
    pub pulmonary_artery: String,
    pub pulmonary_artery_systolic_pressure: String,
    pub max_grad_tricuspidal_regurgitation: String,
    pub pulmonary_artery_right_branch_full: String,
    pub pulmonary_artery_left_branch_full: String,
    pub max_velocity_in_pulmonary_artery: String,
    pub max_grad_in_pulmonary_artery: String,
    pub pulmonary_regurgitation_max_velocity_full: String,
    pub pulmonary_regurgitation_max_grad_full: String,
    pub pulmonary_artery_med_pressure_full: String,
    pub vena: String,
    pub effusion: String,
    pub today: String,
}

pub fn get_selected(msg: &str, options: Vec<&str>) -> String {
    return match Select::new(msg, options).prompt() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

pub fn safe_get_string(msg: &str) -> String {
    loop {
        let inp = match input(msg) {
            Some(v) if v.is_empty() => {
                eprintln!("Тут нельзя ничего не ввести.");
                continue;
            }
            Some(v) => v,
            None => continue,
        };

        return inp;
    }
}

pub fn safe_get_date(msg: &str) -> NaiveDate {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => continue,
        };

        let date = match NaiveDate::parse_from_str(&inp, "%d%m%Y") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        return date;
    }
}

enum ParseNumError {
    Empty,
    Invalid,
}

fn input(msg: &str) -> Option<String> {
    let inp = match Text::new(msg).prompt() {
        Ok(i) => i,
        Err(InquireError::OperationCanceled) => {
            eprintln!("Input cancelled (Ctrl+D)");
            return None;
        }
        Err(InquireError::OperationInterrupted) => {
            eprintln!("Interrupted (Ctrl+C)");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Input error occured: {}", e);
            return None;
        }
    };
    return Some(inp.trim().to_owned());
}

fn parse_num_opt(inp: String, precision: u8) -> Result<f64, ParseNumError> {
    if inp.is_empty() {
        return Err(ParseNumError::Empty);
    }

    let num: i64 = inp.parse().map_err(|_| ParseNumError::Invalid)?;
    return Ok(num as f64 / 10_i32.pow(precision as u32) as f64);
}

pub fn safe_get_num(msg: &str, precision: u8) -> f64 {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => {
                eprintln!("Это обязательное поле, его нельзя пропутить.");
                continue;
            }
        };

        let res: f64 = match inp.parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Некорректный ввод.");
                continue;
            }
        };

        return res as f64 / 10_i32.pow(precision as u32) as f64;
    }
}

pub fn safe_get_num_opt(msg: &str, precision: u8) -> Option<f64> {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => {
                eprintln!("Это обязательное поле, его нельзя пропутить.");
                continue;
            }
        };

        let res: f64 = match parse_num_opt(inp, precision) {
            Ok(v) => v,
            Err(ParseNumError::Empty) => return None,
            Err(_) => {
                eprintln!("Некорректный ввод.");
                continue;
            }
        };

        return Some(res);
    }
}

pub fn prep_num_opt(num: Option<f64>, left: &str, right: &str, precision: u8) -> String {
    match num {
        Some(v) => {
            let middle = format!("{:.*}", precision as usize, v).replace('.', ",");
            format!("{}{}{}", left, middle, right)
        }
        None => "".to_owned(),
    }
}

pub fn prep_num(num: impl ToString) -> String {
    num.to_string().replace('.', ",")
}

pub fn safe_get_num_x_num(msg: &str, precision: u8) -> String {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => {
                eprintln!("Это обязательное поле, его нельзя пропутить.");
                continue;
            }
        };

        let mut inp = inp.split_whitespace();

        let num1: f64 = match inp.next() {
            Some(v) => match v.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Первое число некорректно.");
                    continue;
                }
            },
            None => {
                eprintln!("Введите два числа через пробел.");
                continue;
            }
        };

        let num2: f64 = match inp.next() {
            Some(v) => match v.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Второе число некорректно.");
                    continue;
                }
            },
            None => {
                eprintln!("Введите два числа через пробел.");
                continue;
            }
        };

        if inp.next().is_some() {
            eprintln!("Должно быть только 2 числа.");
            continue;
        }

        return format!(
            "{}×{}",
            prep_num_precise(num1 as f64 / 10_i32.pow(precision as u32) as f64, precision),
            prep_num_precise(num2 as f64 / 10_i32.pow(precision as u32) as f64, precision)
        );
    }
}

pub fn calc_age(birthday: NaiveDate, td: DateTime<Local>) -> i32 {
    let today = td.date_naive();
    let mut age: i32 = today.year() - birthday.year();
    if today.ordinal() < birthday.ordinal() {
        age -= 1;
    }
    age
}

pub fn prep_num_precise(num: f64, precision: u8) -> String {
    format!("{:.*}", precision as usize, num).replace('.', ",")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub save_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let v = get_exe_dir().join("output");
        fs::create_dir(&v).unwrap_or_else(|_| {});
        Self { save_dir: v }
    }
}

fn get_exe_dir() -> PathBuf {
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

    // 1) если файл есть — пробуем прочитать и распарсить
    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str::<Settings>(&text) {
            // (необязательно) можно убедиться, что каталог существует
            let _ = fs::create_dir_all(&settings.save_dir);
            return settings;
        }
        // если JSON битый/не тот — падаем ниже в выбор каталога
    }
    // 2) файла нет или он некорректный → спрашиваем у пользователя
    let save_dir = choose_save_dir_or_default();

    let settings = Settings { save_dir };

    // 3) сохраняем обратно
    save_settings(&path, &settings);

    settings
}

fn choose_save_dir_or_default() -> PathBuf {
    match FileDialog::new()
        .set_title("Выберите каталог для сохранения")
        .pick_folder()
    {
        Some(p) => p,
        None => Settings::default().save_dir,
    }
}

fn save_settings(path: &Path, settings: &Settings) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let text = serde_json::to_string_pretty(settings).unwrap();
    fs::write(path, text).unwrap();
}

pub fn simple_num_depends_of(condition: Option<f64>, msg: &str, precision: u8) -> Option<f64> {
    match condition {
        Some(_) => return Some(safe_get_num(msg, precision)),
        None => return None,
    }
}

// enum Departments {
//     Kdo,
//     Diot,
//     Pulm,
//     Revm,
//     Kho,
//     // "КДО", "ДиОТ", "пульм", "ревм", "КХО"
// }
// impl fmt::Display for Departments {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Departments::Kdo => write!(f, "КДО"),
//             Departments::Diot => write!(f, "ДиОТ"),
//             Departments::Pulm => write!(f, "пульм"),
//             Departments::Revm => write!(f, "ревм"),
//             Departments::Kho => write!(f, "КХО"),
//         }
//     }
// }

pub fn simple_num_depends_of_opt(condition: Option<f64>, msg: &str, precision: u8) -> Option<f64> {
    match condition {
        Some(_) => return safe_get_num_opt(msg, precision),
        None => return None,
    }
}

pub struct StrokeVolume {
    pub value: f64,
    pub auto: bool,
}

pub fn prep_volume(volume: StrokeVolume, precision: u8) -> String {
    let r = format!("{:.*}", precision as usize, volume.value).replace('.', ",");
    if volume.auto {
        r + " мл"
    } else {
        r + " мл (по допплеру)"
    }
}
