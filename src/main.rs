use chrono::{Datelike, Local, NaiveDate};
use inquire::{Select, Text};
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::fs;

pub fn fmt1<S>(v: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let txt = format!("{:.1}", v).replace('.', ",");
    s.serialize_str(&txt)
}

pub fn fmt2<S>(v: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let txt = format!("{:.2}", v).replace('.', ",");
    s.serialize_str(&txt)
}

pub fn fmt_date<S>(d: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&d.format("%d.%m.%Y").to_string())
}

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

    #[serde(serialize_with = "fmt_date")]
    pub birthday: NaiveDate,

    pub department: Department,

    pub cardnum: String,

    pub age: i32,

    pub height: u16,
    pub weight: u16,

    #[serde(serialize_with = "fmt2")]
    pub body_surface_area: f64,

    pub pulse: u16,

    #[serde(serialize_with = "fmt1")]
    pub left_ventricle_diastolic_size: f64,

    #[serde(serialize_with = "fmt1")]
    pub left_ventricle_systolic_size: f64,

    #[serde(serialize_with = "fmt1")]
    pub septum_thickness: f64,

    #[serde(serialize_with = "fmt1")]
    pub posterior_wall_thickness: f64,

    pub left_ventricle_mass: u16,
    pub left_ventricle_mass_index: u16,

    #[serde(serialize_with = "fmt2")]
    pub relative_wall_thickness: f64,

    pub stroke_volume: u16,

    #[serde(serialize_with = "fmt1")]
    pub cardiac_index: f64,

    #[serde(serialize_with = "fmt1")]
    pub cardiac_output: f64,

    pub simpson_end_diastolic_volume: u16,
    pub simpson_end_systolic_volume: u16,
    pub ejection_fraction: u16,

    #[serde(serialize_with = "fmt1")]
    pub aortic_sinus_diameter: f64,

    #[serde(serialize_with = "fmt1")]
    pub ascending_aorta_diameter: f64,
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
    let name = Text::new("ФИО пациента:").prompt().unwrap();

    let birthday = Text::new("Дата рождения пациента:").prompt().unwrap();
    let birthday = NaiveDate::parse_from_str(&birthday, "%d.%m.%Y").unwrap();

    let departments = vec![Department::Kdo, Department::Caop, Department::Diot];
    let department: Department = Select::new("Отделение:", departments).prompt().unwrap();

    let cardnum: String = match department {
        Department::Diot => {
            let aknum = Text::new("АК№:").prompt().unwrap();
            format!("АК№: {}-{}-А", aknum, Local::now().format("%Y"))
        }
        _ => {
            let ibnum = Text::new("ИБ№:").prompt().unwrap();
            format!("ИБ№: {}-{}-C", ibnum, Local::now().format("%y"))
        }
    };

    let left_ventricle_diastolic_size: f64 = Text::new("КДР:")
        .prompt()
        .unwrap()
        .replace(',', ".")
        .parse()
        .unwrap();
    let height = Text::new("Рост:").prompt().unwrap();

    let simpson_end_diastolic_volume: u16 = Text::new("КДО (по Симпсону):")
        .prompt()
        .unwrap()
        .parse()
        .unwrap();
    let simpson_end_systolic_volume: u16 = Text::new("КСО (по Симпсону):")
        .prompt()
        .unwrap()
        .parse()
        .unwrap();

    let out_path = format!("./output/{} {}.docx", &name, Local::now().format("%y%m%d"));

    let ready_data = EchoReport {
        name,

        birthday,

        department,
        cardnum,

        age: calc_age(birthday),

        height: height.parse().unwrap(),
        weight: 82,

        body_surface_area: 2.01,

        pulse: 72,

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

        aortic_sinus_diameter: 3.4,
        ascending_aorta_diameter: 3.1,
    };
    let template_bytes = fs::read("./assets/tplt.docx").unwrap();

    let data: Value = serde_json::to_value(&ready_data).unwrap();

    let rendered_bytes = docx_handlebars::render_template(template_bytes, &data).unwrap();

    fs::create_dir("./output").unwrap_or_else(|_| {});
    fs::write(out_path, rendered_bytes).unwrap();
}
