use chrono::{Datelike, Local, NaiveDate};
use inquire::{InquireError, Select, Text};
use rfd::FileDialog;
use serde::Serialize;
use serde_json::Value;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Department {
    #[serde(rename = "КДО")]
    Kdo,
    #[serde(rename = "ЦАОП")]
    Caop,
    #[serde(rename = "ДиОТ")]
    Diot,
}
impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Department::Kdo => "КДО",
            Department::Caop => "ЦАОП",
            Department::Diot => "ДиОТ",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Serialize)]
pub struct EchoReport {
    pub name: String,

    pub birthday: String,

    pub department: Department,

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
}

fn calc_age(birthday: NaiveDate) -> i32 {
    let today = Local::now().date_naive();
    let mut age: i32 = today.year() - birthday.year();
    if today.ordinal() < birthday.ordinal() {
        age -= 1;
    }
    age
}

fn main() {
    let folder = FileDialog::new()
        .set_title("Выберите каталог для сохранения")
        .pick_folder();

    let mut path = match folder {
        Some(p) => p,
        None => PathBuf::from("./output/"),
    };

    let name = Text::new("ФИО:").prompt().unwrap();

    let birthday = Text::new("Дата рождения (ДДММГГГГ):").prompt().unwrap();
    let birthday = NaiveDate::parse_from_str(&birthday, "%d%m%Y").unwrap();

    let departments = vec![Department::Kdo, Department::Caop, Department::Diot];
    let department: Department = Select::new("Отделение:", departments).prompt().unwrap();

    let cardnum: String = match department {
        Department::Diot => {
            let ibnum = Text::new("ИБ№:").prompt().unwrap();
            format!("ИБ№: {}-{}-C", ibnum, Local::now().format("%y"))
        }
        _ => {
            let aknum = Text::new("АК№:").prompt().unwrap();
            format!("АК№: {}-{}-А", aknum, Local::now().format("%Y"))
        }
    };

    let height: i32 = Text::new("Рост:").prompt().unwrap().parse().unwrap();
    let weight: i32 = Text::new("Вес:").prompt().unwrap().parse().unwrap();
    let pulse: i32 = Text::new("ЧСС:").prompt().unwrap().parse().unwrap();

    let aortic_sinus_diameter: f64 = Text::new("Ао:")
        .prompt()
        .unwrap()
        .replace(",", ".")
        .parse()
        .unwrap();
    let ascending_aorta_diameter: f64 = Text::new("ВА:")
        .prompt()
        .unwrap()
        .replace(",", ".")
        .parse()
        .unwrap();
    let left_atrium: f64 = Text::new("ЛП:")
        .prompt()
        .unwrap()
        .replace(",", ".")
        .parse()
        .unwrap();

    let left_atrium4_ask: String = Text::new("ЛП4:").prompt().unwrap();
    let left_atrium4_prep: Vec<&str> = left_atrium4_ask.split_whitespace().collect();
    let left_atrium4: String = format!("{}×{}", left_atrium4_prep[0], left_atrium4_prep[1]);

    let left_atrium_volume: i32 = Text::new("ЛП V:").prompt().unwrap().parse().unwrap();

    let right_atrium4_ask: String = Text::new("ЛП4:").prompt().unwrap();
    let right_atrium4_prep: Vec<&str> = right_atrium4_ask.split_whitespace().collect();
    let right_atrium4: String = format!("{}×{}", right_atrium4_prep[0], right_atrium4_prep[1]);

    let right_atrium_s: i32 = Text::new("ПП S:").prompt().unwrap().parse().unwrap();
    let right_atrium_volume: i32 = Text::new("ПП V:").prompt().unwrap().parse().unwrap();

    let right_ventricle: f64 = Text::new("ПЗР ПЖ:")
        .prompt()
        .unwrap()
        .replace(",", ".")
        .parse()
        .unwrap();

    let right_ventricle_baz: f64 = Text::new("ПЖ баз:")
        .prompt()
        .unwrap()
        .replace(",", ".")
        .parse()
        .unwrap();

    // let ight_ventricle_medium = Text::new("ПЖ ср:")
    //     .prompt()
    //     .unwrap();

    // пока не надо начало
    let left_ventricle_diastolic_size: f64 = Text::new("КДР:")
        .prompt()
        .unwrap()
        .replace(',', ".")
        .parse()
        .unwrap();

    let simpson_end_diastolic_volume: i32 = Text::new("КДО (по Симпсону):")
        .prompt()
        .unwrap()
        .parse()
        .unwrap();
    let simpson_end_systolic_volume: i32 = Text::new("КСО (по Симпсону):")
        .prompt()
        .unwrap()
        .parse()
        .unwrap();
    // пока не надо конец

    //расчёты начало
    let body_surface_area: f64 =
        f64::powf(height as f64, 0.725) * f64::powf(weight as f64, 0.425) * 0.007;

    let left_atrium_index: f64 = left_atrium_volume as f64 / body_surface_area;

    let age = calc_age(birthday);
    //расчёты конец

    let out_filename: String = format!("{} {}.docx", &name, Local::now().format("%y%m%d"));

    let ready_data = EchoReport {
        name,

        birthday,

        department,
        cardnum,

        age: calc_age(birthday),

        height,
        weight,
        pulse,

        aortic_sinus_diameter,
        ascending_aorta_diameter,

        left_atrium,
        left_atrium4,
        left_atrium_volume,
        left_atrium_index,

        right_atrium4,
        right_atrium_s,
        right_atrium_volume,

        right_ventricle,
        right_ventricle_baz,
        body_surface_area,

        left_ventricle_diastolic_size,

        left_ventricle_systolic_size: 3.4,
        septum_thickness: 0.9,
        posterior_wall_thickness: 1.0,

        left_ventricle_mass: 180,
        left_ventricle_mass_index: 92,

        relative_wall_thickness: 0.38,

        stroke_volume: 75,

        cardiac_index: 2.7,
        cardiac_output: 5.4,

        simpson_end_diastolic_volume,
        simpson_end_systolic_volume,
        ejection_fraction: 62,

        right_ventricle_medium: Some("fo".to_string()),
    };
    let template_bytes = fs::read("./assets/tplt.docx").unwrap();

    let data: Value = serde_json::to_value(&ready_data).unwrap();

    let rendered_bytes = docx_handlebars::render_template(template_bytes, &data).unwrap();

    fs::create_dir(&path).unwrap_or_else(|_| {});
    path.push(out_filename);
    fs::write(path, rendered_bytes).unwrap();
}

fn safe_get_number(msg: &str, precision: u8) -> f64 {
    loop {
        let input = match Text::new(msg).prompt() {
            Ok(i) => i,
            Err(InquireError::OperationCanceled) => {
                eprintln!("Interrupted (Ctrl+D)");
                std::process::exit(0);
            }
            Err(InquireError::OperationInterrupted) => {
                eprintln!("Interrupted (Ctrl+C)");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Input error occured: {}", e);
                continue;
            }
        };

        let num: i32 = match input.parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Parsing error occured: {}", e);
                continue;
            }
        };

        return num as f64 / 10.0;
    }
}

