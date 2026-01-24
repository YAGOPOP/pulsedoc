use chrono::{Datelike, Local, NaiveDate};
use inquire::{InquireError, Select, Text};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env::current_exe,
    fs::{self, create_dir, read_to_string},
    path::{Path, PathBuf},
    process::exit,
};

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
    pub right_ventricle_medium: String,
    pub right_ventricle_medium_full: String,
    pub right_ventricle_wall_thickness_full: String,
    pub tapse_full: String,
}

fn main() {
    let cur_settings = load_settings();

    let name: String = safe_get_string("ФИО:");
    let birthday: NaiveDate = safe_get_date("Дата рождения (ДДММГГ):");

    let departments = vec!["ДиОТ", "ЦАОП", "КДО"];
    let department = match Select::new("Отделение:", departments).prompt() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let cardnum: String = match department {
        "ДиОТ" => {
            let ibnum: f64 = safe_get_num("ИБ№:", 0);
            format!("ИБ№: {}-{}-C", ibnum, Local::now().format("%y"))
        }
        _ => {
            let aknum: f64 = safe_get_num("АК№:", 0);
            format!("АК№: {}-{}-А", aknum, Local::now().format("%Y"))
        }
    };

    let height: f64 = safe_get_num("Рост:", 0);
    let weight: f64 = safe_get_num("Вес:", 0);
    let pulse: f64 = safe_get_num("ЧСС:", 0);
    let aortic_sinus_diameter: f64 = safe_get_num("Ао (*10^(-1)):", 1);
    let ascending_aorta_diameter: f64 = safe_get_num("ВА (*10^(-1)):", 1);
    let left_atrium: f64 = safe_get_num("ЛП:", 1);
    let left_atrium4: String = safe_get_num_x_num("ЛП4:", 0);
    let left_atrium_volume: f64 = safe_get_num("ЛП V:", 0);
    let right_atrium4: String = safe_get_num_x_num("ПП4:", 0);
    let right_atrium_s: f64 = safe_get_num("ПП S:", 0);
    let right_atrium_volume: f64 = safe_get_num("ПП V:", 0);
    let right_ventricle: f64 = safe_get_num("ПЗР ПЖ (*10^(-1)):", 1);
    let right_ventricle_baz: f64 = safe_get_num("ПЖ баз (*10^(-1)):", 1);

    let right_ventricle_medium: Option<f64> =
        safe_get_num_opt("ПЖ ср (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);
    let right_ventricle_wall_thickness: Option<f64> =
        safe_get_num_opt("ПСПЖ (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);
    let tapse = safe_get_num_opt("TAPSE (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);

    // // пока не надо начало
    // let left_ventricle_diastolic_size: f64 = Text::new("КДР:")
    //     .prompt()
    //     .unwrap()
    //     .replace(',', ".")
    //     .parse()
    //     .unwrap();

    // let simpson_end_diastolic_volume: i32 = Text::new("КДО (по Симпсону):")
    //     .prompt()
    //     .unwrap()
    //     .parse()
    //     .unwrap();
    // let simpson_end_systolic_volume: i32 = Text::new("КСО (по Симпсону):")
    //     .prompt()
    //     .unwrap()
    //     .parse()
    //     .unwrap();
    // // пока не надо конец

    //расчёты начало
    let body_surface_area: f64 =
        f64::powf(height as f64, 0.725) * f64::powf(weight as f64, 0.425) * 0.007;

    let left_atrium_index: f64 = left_atrium_volume as f64 / body_surface_area;

    let age = calc_age(birthday);
    //расчёты конец

    // имя файла
    let out_filename: String = format!("{} {}.docx", &name, Local::now().format("%y%m%d"));
    // имя файла

    // аллокации начало
    // let right_ventricle_medium_full = format!(", средний {} см (N< 3,5 см)");
    // let right_ventricle_wall_thickness_full = format!();
    // let tapse = format!();
    // аллокации конец

    let ready_data = EchoReport {
        name,
        birthday: birthday.format("%d.%m.%Y").to_string(),
        department: department.to_owned(),
        aortic_sinus_diameter: prep_num(aortic_sinus_diameter),
        ascending_aorta_diameter: prep_num(ascending_aorta_diameter),
        cardnum,
        cardiac_index: "TEMPORARY_PLACEHOLDER".to_owned(),
        cardiac_output: "TEMPORARY_PLACEHOLDER".to_owned(),
        ejection_fraction: "TEMPORARY_PLACEHOLDER".to_owned(),
        height: prep_num(height),
        left_atrium: prep_num(left_atrium),
        left_atrium4,
        left_atrium_volume: prep_num(left_atrium_volume),
        left_ventricle_diastolic_size: "TEMPORARY_PLACEHOLDER".to_owned(),
        left_ventricle_mass: "TEMPORARY_PLACEHOLDER".to_owned(),
        left_ventricle_mass_index: "TEMPORARY_PLACEHOLDER".to_owned(),
        left_ventricle_systolic_size: "TEMPORARY_PLACEHOLDER".to_owned(),
        posterior_wall_thickness: "TEMPORARY_PLACEHOLDER".to_owned(),
        pulse: prep_num(pulse),
        relative_wall_thickness: "TEMPORARY_PLACEHOLDER".to_owned(),
        right_atrium4,
        right_atrium_s: prep_num(right_atrium_s),
        right_atrium_volume: prep_num(right_atrium_volume),
        right_ventricle: prep_num(right_ventricle),
        right_ventricle_baz: prep_num(right_ventricle_baz),
        right_ventricle_medium: "TEMPORARY_PLACEHOLDER".to_owned(),
        septum_thickness: "TEMPORARY_PLACEHOLDER".to_owned(),
        simpson_end_diastolic_volume: "TEMPORARY_PLACEHOLDER".to_owned(),
        simpson_end_systolic_volume: "TEMPORARY_PLACEHOLDER".to_owned(),
        stroke_volume: "TEMPORARY_PLACEHOLDER".to_owned(),
        weight: prep_num(weight),
        right_ventricle_medium_full: prep_num_opt(
            right_ventricle_medium,
            ", средний ",
            " см (N< 3,5 см)",
            1,
        ),
        right_ventricle_wall_thickness_full: prep_num_opt(
            right_ventricle_wall_thickness,
            ". Толщина передней стенки ПЖ: ",
            " см (N<0,5 см)",
            1,
        ),
        tapse_full: prep_num_opt(tapse, ". TAPSE: ", " см (N>=1,7 см)", 1),

        age: prep_num(age),
        body_surface_area: prep_num_precise(body_surface_area, 1),
        left_atrium_index: prep_num_precise(left_atrium_index, 1),
    };

    // // работа с файлами начало

    let template_bytes = fs::read("./assets/tplt.docx").unwrap();

    let data: Value = serde_json::to_value(&ready_data).unwrap();

    let rendered_bytes = docx_handlebars::render_template(template_bytes, &data).unwrap();

    let my_path = cur_settings.save_dir;
    fs::write(my_path.join(out_filename), rendered_bytes).unwrap();
}

fn safe_get_string(msg: &str) -> String {
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

fn safe_get_date(msg: &str) -> NaiveDate {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => continue,
        };

        let date = match NaiveDate::parse_from_str(&inp, "%d%m%y") {
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
            exit(0);
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

fn safe_get_num(msg: &str, precision: u8) -> f64 {
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

fn safe_get_num_opt(msg: &str, precision: u8) -> Option<f64> {
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

fn prep_num_opt(num: Option<f64>, left: &str, right: &str, precision: u8) -> String {
    match num {
        Some(v) => {
            let middle = format!("{:.*}", precision as usize, v).replace('.', ",");
            format!("{}{}{}", left, middle, right)
        }
        None => "".to_owned(),
    }
}

fn prep_num(num: impl ToString) -> String {
    num.to_string().replace('.', ",")
}

fn safe_get_num_x_num(msg: &str, precision: u8) -> String {
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
            prep_num(num1 as f64 / 10_i32.pow(precision as u32) as f64),
            prep_num(num2 as f64 / 10_i32.pow(precision as u32) as f64)
        );
    }
}

fn calc_age(birthday: NaiveDate) -> i32 {
    let today = Local::now().date_naive();
    let mut age: i32 = today.year() - birthday.year();
    if today.ordinal() < birthday.ordinal() {
        age -= 1;
    }
    age
}

fn prep_num_precise(num: f64, precision: u8) -> String {
    format!("{:.*}", precision as usize, num).replace('.', ",")
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    save_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let v = get_exe_dir().join("output");
        create_dir(&v).unwrap_or_else(|_| {});
        Self { save_dir: v }
    }
}

fn get_exe_dir() -> PathBuf {
    match current_exe() {
        Ok(exe) => match exe.parent() {
            Some(dir) => dir.to_path_buf(),
            None => {
                eprintln!("Ошибка: некорректное расположение исполняемого файла");
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Ошибка: путь исполняемого файла не найден. ({})", e);
            exit(1);
        }
    }
}

fn load_settings() -> Settings {
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
