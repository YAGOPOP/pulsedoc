use crate::promptget::{
    AutoValue, PreciseNum, RenderToString, ask_selection, calc_age, get_date, get_int, get_int_if,
    get_num, get_num_if, get_num_opt, get_num_opt_if, get_num_x_num, get_string,
};
use crate::reporttypes::{CalculatedReportData, RawReportData};
use chrono::{DateTime, Local};
use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Department {
    Kdo,
    Diot,
    Pulm,
    Revm,
    Kho,
}

impl Department {
    pub fn label(self) -> &'static str {
        match self {
            Department::Kdo => "КДО",
            Department::Diot => "ДиОТ",
            Department::Pulm => "пульм",
            Department::Revm => "ревм",
            Department::Kho => "КХО",
        }
    }
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum ValveLeaflets {
    Normal,
    Thickened,
    ThickenedWithSmallCalcifications,
    Calcified,
}

impl ValveLeaflets {
    pub fn text(self) -> &'static str {
        match self {
            ValveLeaflets::Normal => "без особенностей",
            ValveLeaflets::Thickened => "уплотнены",
            ValveLeaflets::ThickenedWithSmallCalcifications => {
                "уплотнены, с включением мелких кальцинатов"
            }
            ValveLeaflets::Calcified => "уплотнены, кальцинированы",
        }
    }
}

impl fmt::Display for ValveLeaflets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl RenderToString for ValveLeaflets {
    fn render_to_string(&self) -> String {
        format!("{}. ", self.text())
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Stenosis {
    No,
    Yes,
}

impl Stenosis {
    pub fn text(self) -> &'static str {
        match self {
            Stenosis::No => "нет",
            Stenosis::Yes => "есть",
        }
    }

    pub fn is_yes(self) -> bool {
        matches!(self, Stenosis::Yes)
    }
}

impl fmt::Display for Stenosis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MitVal {
    Calcinate,
    Calcification,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum YesNo {
    No,
    Yes,
}

impl YesNo {
    pub fn text(self) -> &'static str {
        match self {
            YesNo::No => "Нет",
            YesNo::Yes => "Да",
        }
    }

    pub fn render_to_string(&self, ctxt: MitVal) -> String {
        match self {
            Self::No => "".to_owned(),
            Self::Yes => match ctxt {
                MitVal::Calcinate => "Кальцинат в основании задней створки. ".to_owned(),
                MitVal::Calcification => {
                    "Кальциноз основания задней створки, фиброзного кольца. ".to_owned()
                }
            },
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum TdiRelation {
    ELessThanA,
    EGreaterThanA,
}

impl TdiRelation {
    pub fn text(self) -> &'static str {
        match self {
            TdiRelation::ELessThanA => "e<a",
            TdiRelation::EGreaterThanA => "e>a",
        }
    }
}

impl fmt::Display for TdiRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum PericardialEffusion {
    NotDetected,
    DetectedDetailed,
}

impl PericardialEffusion {
    pub fn text(self) -> &'static str {
        match self {
            PericardialEffusion::NotDetected => "не выявлен",
            PericardialEffusion::DetectedDetailed => {
                "эхонегативное пространство по задней стенке левого желудочка до см, по боковой стенке левого желудочка до см, по боковой стенке правого желудочка см, по передней стенке правого желудочка см. по верхне-латеральному краю правого предсердия до см"
            }
        }
    }
}

impl fmt::Display for PericardialEffusion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl RenderToString for PericardialEffusion {
    fn render_to_string(&self) -> String {
        format!("{}. ", self.text())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardNumber {
    Ak(i64),
    Ib(i64),
}

impl CardNumber {
    pub fn render_to_string(&self, today: DateTime<Local>) -> String {
        match self {
            Self::Ib(n) => format!("ИБ№: {}-{}-C", n, today.format("%y")),
            Self::Ak(n) => format!("АК№: {}-{}-А", n, today.format("%Y")),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum AtriumPressure {
    Plus3,
    Other,
}

impl fmt::Display for AtriumPressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus3 => write!(f, "3"),
            Self::Other => write!(f, "иное"),
        }
    }
}

impl AtriumPressure {
    fn is_other(&self) -> bool {
        match self {
            Self::Other => true,
            Self::Plus3 => false,
        }
    }
}

impl RawReportData {
    pub fn gather() -> Self {
        let name = get_string("ФИО");
        let birthday = get_date("Дата рождения");
        let department: Department = ask_selection("Отделение");

        let card_number: CardNumber = match department {
            Department::Kdo => CardNumber::Ak(get_int("АК№")),
            _ => CardNumber::Ib(get_int("ИБ№")),
        };

        let height = get_num("Рост", 0);
        let weight = get_num("Вес", 0);
        let pulse = get_num("ЧСС", 0);
        let aortic_sinus_diameter = get_num("Ао", 1);
        let ascending_aorta_diameter = get_num("ВА", 1);
        let left_atrium = get_num("ЛП", 1);
        let left_atrium4 = get_num_x_num("ЛП4", 1);
        let left_atrium_volume = get_num("ЛП V", 0);
        let right_atrium4 = get_num_x_num("ПП4", 1);
        let right_atrium_s = get_num("ПП S", 0);
        let right_atrium_volume = get_num("ПП V", 0);
        let right_ventricle = get_num("ПЗР ПЖ", 1);
        let right_ventricle_baz = get_num("ПЖ баз", 1);
        let right_ventricle_medium = get_num_opt("ПЖ ср", 1);
        let right_ventricle_wall_thickness = get_num_opt("ПСПЖ", 1);
        let tapse = get_num_opt("TAPSE", 1);
        let left_ventricle_diastolic_size = get_num("КДР", 1);
        let left_ventricle_systolic_size = get_num("КСР", 1);
        let septum_thickness = get_num("МЖП", 1);
        let septum_thickness_baz = get_num_opt("МЖП баз", 1);
        let posterior_wall_thickness = get_num("ЗС", 1);
        let simpson_end_diastolic_volume = get_num("КДО (по Симпсону)", 0);
        let simpson_end_systolic_volume = get_num("КСО (по Симпсону)", 0);

        let stroke_volume = get_num_opt("УО", 0);

        let shutters_aortal: ValveLeaflets = ask_selection("АК");
        let opening_amplitude = get_num("Амплитуда раскрытия", 1);
        let max_velocity_aortal = get_num("Макс скорость", 1);
        let max_grad_aortal = get_num("Макс градиент", 0);
        let stenosis: Stenosis = ask_selection("Стеноз");
        let mid_grad = get_num_if(stenosis.is_yes(), "Средний градиент", 0);
        let s_doppler = get_num_if(stenosis.is_yes(), "Площадь по допплеру", 1);
        let s_planim = get_num_if(stenosis.is_yes(), "Площадь планиметрически", 1);
        let presh_time = get_num_opt("PHT", 0);
        let vena_contracta = get_num_if(presh_time.is_some(), "VC АР", 1);
        let max_velocity_vt =
            get_num_opt_if(septum_thickness_baz.is_some(), "ВТЛЖ Макс скорость", 1);
        let max_grad_vt = get_num_if(max_velocity_vt.is_some(), "ВТЛЖ макс градиент", 0);
        let shutters_mitral: ValveLeaflets = ask_selection("МК");
        let calts_back_sash: YesNo = ask_selection("Кальцинат в основании задней створки");
        let posterior_leaflet_base_calcification: YesNo =
            ask_selection("Кальциноз основания задней створки, фиброзного кольца");
        let peak_e = get_num("МК: Е", 0);
        let peak_a = get_num("А", 0);
        let tdi_vel: TdiRelation = ask_selection("TDI");
        let e_sept = get_num("E sept", 0);
        let e_lat = get_num("E’ lat", 0);
        let max_velocity_mitral_valve = get_num_opt("МК Макс скорость", 1);
        let max_grad_mitral_valve =
            get_num_if(max_velocity_mitral_valve.is_some(), "МК Макс градиент", 1);
        let mid_grad_mitral_valve = get_num_if(
            max_velocity_mitral_valve.is_some(),
            "МК Средний градиент",
            1,
        );
        let max_velocity_tricuspidal_regurgitation = get_num("ТК Макс скорость ТР", 1);
        let max_grad_tricuspidal_regurgitation = get_num("ТК макс градиент ТР", 0);

        let right_atrium_pressure_choice: AtriumPressure = ask_selection(&format!(
            "СДЛА: к {} прибавить",
            max_grad_tricuspidal_regurgitation
        ));
        let right_atrium_pressure = get_int_if(right_atrium_pressure_choice.is_other(), "Иное");

        let pulmonary_artery = get_num("Диаметр ЛА", 1);
        let pulmonary_artery_right_branch = get_num_opt("Правая ветвь ЛА", 1);
        let pulmonary_artery_left_branch =
            get_num_if(pulmonary_artery_right_branch.is_some(), "Левая ветвь ЛА", 1);
        let max_velocity_in_pulmonary_artery = get_num("ЛА макс. скорость", 1);
        let max_grad_in_pulmonary_artery = get_num("ЛА макс градиент", 0);
        let pulmonary_regurgitation_max_velocity = get_num_opt("ЛР макс. скорость", 1);
        let pulmonary_regurgitation_max_grad = get_num_if(
            pulmonary_regurgitation_max_velocity.is_some(),
            "ЛР макс градиент",
            0,
        );
        let vena = get_num("НПВ", 1);
        let effusion: PericardialEffusion = ask_selection("Перикардиальный выпот");

        Self {
            name,
            birthday,
            department,
            card_number,

            height,
            weight,
            pulse,

            aortic_sinus_diameter,
            ascending_aorta_diameter,

            left_atrium,
            left_atrium4,
            left_atrium_volume,

            right_atrium4,
            right_atrium_s,
            right_atrium_volume,

            right_ventricle,
            right_ventricle_baz,
            right_ventricle_medium,
            right_ventricle_wall_thickness,
            tapse,

            left_ventricle_diastolic_size,
            left_ventricle_systolic_size,
            septum_thickness,
            septum_thickness_baz,
            posterior_wall_thickness,

            simpson_end_diastolic_volume,
            simpson_end_systolic_volume,
            stroke_volume,

            shutters_aortal,
            opening_amplitude,
            max_velocity_aortal,
            max_grad_aortal,

            mid_grad,
            s_doppler,
            s_planim,

            presh_time,
            vena_contracta,

            max_velocity_vt,
            max_grad_vt,

            shutters_mitral,
            calts_back_sash,
            posterior_leaflet_base_calcification,

            peak_e,
            peak_a,
            tdi_vel,
            e_sept,
            e_lat,

            max_velocity_mitral_valve,
            max_grad_mitral_valve,
            mid_grad_mitral_valve,

            max_velocity_tricuspidal_regurgitation,
            max_grad_tricuspidal_regurgitation,
            right_atrium_pressure,

            pulmonary_artery,
            pulmonary_artery_right_branch,
            pulmonary_artery_left_branch,

            max_velocity_in_pulmonary_artery,
            max_grad_in_pulmonary_artery,

            pulmonary_regurgitation_max_velocity,
            pulmonary_regurgitation_max_grad,

            vena,
            effusion,
        }
    }
}

impl CalculatedReportData {
    pub fn from_raw(raw: &RawReportData, today: DateTime<Local>) -> CalculatedReportData {
        // --- базовые вычисления (как в старом main.rs) ---

        let height_v = raw.height.value();
        let weight_v = raw.weight.value();

        let body_surface_area: f64 =
            f64::powf(height_v as f64, 0.725) * f64::powf(weight_v as f64, 0.425) * 0.007;

        let left_atrium_index: f64 = raw.left_atrium_volume.value() / body_surface_area;

        let age: i32 = calc_age(raw.birthday, today);

        let edv = raw.simpson_end_diastolic_volume.value();
        let esv = raw.simpson_end_systolic_volume.value();

        let ejection_fraction: f64 = (edv - esv) / edv * 100.0;

        let lvidd = raw.left_ventricle_diastolic_size.value();
        let ivs = raw.septum_thickness.value();
        let pw = raw.posterior_wall_thickness.value();

        let left_ventricle_mass: f64 =
            0.8 * (1.04 * ((lvidd + ivs + pw).powi(3) - lvidd.powi(3))) + 0.6;

        let left_ventricle_mass_index: f64 = left_ventricle_mass / body_surface_area;

        let relative_wall_thickness: f64 = 2.0 * pw / lvidd;

        // если вручную то по допплеру, иначе авто по Симпсону
        let stroke_volume = match raw.stroke_volume {
            Some(v) => AutoValue {
                value: v,
                auto: false,
            },
            None => AutoValue {
                value: PreciseNum::from_float(edv - esv, 0),
                auto: true,
            },
        };
        let sv = stroke_volume.value.value();

        let cardiac_output: f64 = raw.pulse.value() * sv / 1000.0;
        let cardiac_index: f64 = cardiac_output / body_surface_area;

        let peak_e_div_peak_a: f64 = raw.peak_e.value() / raw.peak_a.value();

        let e_div_e_aps: f64 =
            raw.peak_e.value() / ((raw.e_sept.value() + raw.e_lat.value()) / 2.0);

        // В старом коде это было right_atrium_pressure.
        // Сейчас у тебя хранится "что прибавить" как Option<i64>.
        let right_atrium_pressure = raw.right_atrium_pressure.unwrap_or(3);
        let rap = right_atrium_pressure as f64;

        let pulmonary_artery_systolic_pressure: f64 =
            raw.max_grad_tricuspidal_regurgitation.value() + rap;

        let pulmonary_artery_med_pressure: Option<PreciseNum> =
            match raw.pulmonary_regurgitation_max_grad {
                Some(v) => Some(PreciseNum::from_float(v.value() + rap, 0)),
                None => None,
            };

        // --- сборка результата ---

        Self {
            // из RawReportData
            name: raw.name.clone(),
            birthday: raw.birthday,
            department: raw.department,
            cardnum: raw.card_number,

            height: raw.height,
            weight: raw.weight,
            pulse: raw.pulse,

            aortic_sinus_diameter: raw.aortic_sinus_diameter,

            left_ventricle_diastolic_size: raw.left_ventricle_diastolic_size,
            left_ventricle_systolic_size: raw.left_ventricle_systolic_size,
            septum_thickness: raw.septum_thickness,
            posterior_wall_thickness: raw.posterior_wall_thickness,

            simpson_end_diastolic_volume: raw.simpson_end_diastolic_volume,
            simpson_end_systolic_volume: raw.simpson_end_systolic_volume,

            ascending_aorta_diameter: raw.ascending_aorta_diameter,

            left_atrium: raw.left_atrium,
            left_atrium4: raw.left_atrium4,
            left_atrium_volume: raw.left_atrium_volume,

            right_atrium_s: raw.right_atrium_s,
            right_atrium4: raw.right_atrium4,
            right_atrium_volume: raw.right_atrium_volume,

            right_ventricle: raw.right_ventricle,
            right_ventricle_baz: raw.right_ventricle_baz,
            right_ventricle_medium: raw.right_ventricle_medium, // было: *_full
            right_ventricle_wall_thickness: raw.right_ventricle_wall_thickness, // было: *_full
            tapse: raw.tapse,                                   // было: *_full
            septum_thickness_baz: raw.septum_thickness_baz,     // было: *_full

            shutters_aortal: raw.shutters_aortal,
            opening_amplitude: raw.opening_amplitude,
            max_velocity: raw.max_velocity_aortal, // в Raw это max_velocity_aortal
            max_grad: raw.max_grad_aortal,         // в Raw это max_grad_aortal

            mid_grad: raw.mid_grad,   // было: *_full
            s_doppler: raw.s_doppler, // было: *_full
            s_planim: raw.s_planim,   // было: *_full

            presh_time: raw.presh_time,           // было: *_full
            vena_contracta: raw.vena_contracta,   // было: *_full
            max_velocity_vt: raw.max_velocity_vt, // было: *_full
            max_grad_vt: raw.max_grad_vt,         // было: *_full

            shutters_mitral: raw.shutters_mitral,

            peak_e: raw.peak_e,
            peak_a: raw.peak_a,

            tdi_vel: raw.tdi_vel,

            e_sept: raw.e_sept,
            e_lat: raw.e_lat,

            max_velocity_mitral_valve: raw.max_velocity_mitral_valve, // было: *_full
            max_grad_mitral_valve: raw.max_grad_mitral_valve,         // было: *_full
            mid_grad_mitral_valve: raw.mid_grad_mitral_valve,         // было: *_full

            calts_back_sash: raw.calts_back_sash,
            posterior_leaflet_base_calcification: raw.posterior_leaflet_base_calcification,

            max_velocity_tricuspidal_regurgitation: raw.max_velocity_tricuspidal_regurgitation,

            pulmonary_artery: raw.pulmonary_artery,

            max_grad_tricuspidal_regurgitation: raw.max_grad_tricuspidal_regurgitation,

            pulmonary_artery_right_branch: raw.pulmonary_artery_right_branch, // было: *_full
            pulmonary_artery_left_branch: raw.pulmonary_artery_left_branch,   // было: *_full

            max_velocity_in_pulmonary_artery: raw.max_velocity_in_pulmonary_artery,
            max_grad_in_pulmonary_artery: raw.max_grad_in_pulmonary_artery,

            pulmonary_regurgitation_max_velocity: raw.pulmonary_regurgitation_max_velocity, // было: *_full
            pulmonary_regurgitation_max_grad: raw.pulmonary_regurgitation_max_grad, // было: *_full

            vena: raw.vena,
            effusion: raw.effusion,

            // вычисляемые значения
            age,
            body_surface_area,
            left_atrium_index,
            ejection_fraction,
            left_ventricle_mass,
            left_ventricle_mass_index,
            relative_wall_thickness,
            stroke_volume,
            cardiac_output,
            cardiac_index,
            peak_e_div_peak_a,
            e_div_e_aps,
            pulmonary_artery_systolic_pressure,
            pulmonary_artery_med_pressure, // было: *_full
            today,
        }
    }
}
