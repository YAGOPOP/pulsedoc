use crate::promptget::{AutoValue, NumXNum, PreciseNum, RenderToString, render_to_string};
use crate::report::{
    CardNumber, Department, MitVal, PericardialEffusion, TdiRelation, ValveLeaflets, YesNo,
};
use chrono::{DateTime, Local, NaiveDate};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RawReportData {
    // 1) ФИО
    pub name: String,

    // 2) Дата рождения
    pub birthday: NaiveDate,

    // 3) Отделение
    pub department: Department,

    // 4) АК№/ИБ№ (вводится в зависимости от отделения)
    pub card_number: CardNumber,

    // 5) Рост
    pub height: PreciseNum, // p=0

    // 6) Вес
    pub weight: PreciseNum, // p=0

    // 7) ЧСС
    pub pulse: PreciseNum, // p=0

    // 8) Ао (p=1)
    pub aortic_sinus_diameter: PreciseNum,

    // 9) ВА (p=1)
    pub ascending_aorta_diameter: PreciseNum,

    // 10) ЛП (p=1)
    pub left_atrium: PreciseNum,

    // 11) ЛП4 (2 числа, p=1)
    pub left_atrium4: NumXNum,

    // 12) ЛП V (p=0)
    pub left_atrium_volume: PreciseNum,

    // 13) ПП4 (2 числа, p=1)
    pub right_atrium4: NumXNum,

    // 14) ПП S (p=0)
    pub right_atrium_s: PreciseNum,

    // 15) ПП V (p=0)
    pub right_atrium_volume: PreciseNum,

    // 16) ПЗР ПЖ (p=1)
    pub right_ventricle: PreciseNum,

    // 17) ПЖ баз (p=1)
    pub right_ventricle_baz: PreciseNum,

    // 18) ПЖ ср (p=1, optional)
    pub right_ventricle_medium: Option<PreciseNum>,

    // 19) ПСПЖ (p=1, optional)
    pub right_ventricle_wall_thickness: Option<PreciseNum>,

    // 20) TAPSE (p=1, optional)
    pub tapse: Option<PreciseNum>,

    // 21) КДР (p=1)
    pub left_ventricle_diastolic_size: PreciseNum,

    // 22) КСР (p=1)
    pub left_ventricle_systolic_size: PreciseNum,

    // 23) МЖП (p=1)
    pub septum_thickness: PreciseNum,

    // 24) МЖП баз (p=1, optional)
    pub septum_thickness_baz: Option<PreciseNum>,

    // 25) ЗС (p=1)
    pub posterior_wall_thickness: PreciseNum,

    // 26) КДО по Симпсону (p=0)
    pub simpson_end_diastolic_volume: PreciseNum,

    // 27) КСО по Симпсону (p=0)
    pub simpson_end_systolic_volume: PreciseNum,

    // 28) УО (p=0, optional)
    pub stroke_volume: Option<PreciseNum>,

    // 29) АК (состояние створок)
    pub shutters_aortal: ValveLeaflets,

    // 30) Амплитуда раскрытия (p=1)
    pub opening_amplitude: PreciseNum,

    // 31) Макс скорость (p=1)
    pub max_velocity_aortal: PreciseNum,

    // 32) Макс градиент (p=0)
    pub max_grad_aortal: PreciseNum,

    // 33) Стеноз (нет/есть)
    // pub stenosis: Stenosis,

    // 34) Средний градиент (p=0, только если stenosis=Yes)
    pub mid_grad: Option<PreciseNum>,

    // 35) Площадь по допплеру (p=1, только если stenosis=Yes)
    pub s_doppler: Option<PreciseNum>,

    // 36) Площадь планиметрически (p=1, только если stenosis=Yes)
    pub s_planim: Option<PreciseNum>,

    // 37) PHT (p=0, optional)
    pub presh_time: Option<PreciseNum>,

    // 38) VC АР (p=1, спрашивается только если presh_time.is_some())
    pub vena_contracta: Option<PreciseNum>,

    // 39) ВТЛЖ Макс скорость (p=1, спрашивается только если septum_thickness_baz.is_some())
    pub max_velocity_vt: Option<PreciseNum>,

    // 40) ВТЛЖ макс градиент (p=0, спрашивается только если max_velocity_vt.is_some())
    pub max_grad_vt: Option<PreciseNum>,

    // 41) МК (состояние створок)
    pub shutters_mitral: ValveLeaflets,

    // 42) Кальцинат в основании задней створки (Да/Нет)
    pub calts_back_sash: YesNo,

    // 43) Кальциноз основания задней створки/кольца (Да/Нет)
    pub posterior_leaflet_base_calcification: YesNo,

    // 44) МК: E (p=0)
    pub peak_e: PreciseNum,

    // 45) A (p=0)
    pub peak_a: PreciseNum,

    // 46) TDI (e<a / e>a) — у тебя строкой, оставим строкой
    pub tdi_vel: TdiRelation,

    // 47) E sept (p=0)
    pub e_sept: PreciseNum,

    // 48) E’ lat (p=0)
    pub e_lat: PreciseNum,

    // 49) МК Макс скорость (p=1, optional)
    pub max_velocity_mitral_valve: Option<PreciseNum>,

    // 50) МК Макс градиент (p=1, зависит от max_velocity_mitral_valve)
    pub max_grad_mitral_valve: Option<PreciseNum>,

    // 51) МК Средний градиент (p=1, зависит от max_velocity_mitral_valve)
    pub mid_grad_mitral_valve: Option<PreciseNum>,

    // 52) ТК Макс скорость ТР (p=1)
    pub max_velocity_tricuspidal_regurgitation: PreciseNum,

    // 53) ТК макс градиент ТР (p=0)
    pub max_grad_tricuspidal_regurgitation: PreciseNum,

    // 54) "прибавить 3?" (да / иное)
    pub right_atrium_pressure: Option<i64>,

    // 55) Диаметр ЛА (p=1)
    pub pulmonary_artery: PreciseNum,

    // 56) Правая ветвь ЛА (p=1, optional)
    pub pulmonary_artery_right_branch: Option<PreciseNum>,

    // 57) Левая ветвь ЛА (p=1, спрашивается только если правая ветвь введена)
    pub pulmonary_artery_left_branch: Option<PreciseNum>,

    // 58) ЛА макс скорость (p=1)
    pub max_velocity_in_pulmonary_artery: PreciseNum,

    // 59) ЛА макс градиент (p=0)
    pub max_grad_in_pulmonary_artery: PreciseNum,

    // 60) ЛР макс. скорость (p=1, optional)
    pub pulmonary_regurgitation_max_velocity: Option<PreciseNum>,

    // 61) ЛР макс градиент (p=0, спрашивается только если скорость ЛР введена)
    pub pulmonary_regurgitation_max_grad: Option<PreciseNum>,

    // 62) НПВ (p=1)
    pub vena: PreciseNum,

    // 63) Перикардиальный выпот (выбор из списка)
    pub effusion: PericardialEffusion,
}

#[derive(Debug, Clone)]
pub struct CalculatedReportData {
    // --- поля из RawReportData (в программных типах), но с именами как в старом EchoReport ---
    pub name: String,
    pub birthday: NaiveDate,
    pub department: Department,
    pub cardnum: CardNumber,

    pub height: PreciseNum, // p=0
    pub weight: PreciseNum, // p=0
    pub pulse: PreciseNum,  // p=0

    pub aortic_sinus_diameter: PreciseNum, // p=1

    pub left_ventricle_diastolic_size: PreciseNum, // p=1
    pub left_ventricle_systolic_size: PreciseNum,  // p=1
    pub septum_thickness: PreciseNum,              // p=1
    pub posterior_wall_thickness: PreciseNum,      // p=1

    pub simpson_end_diastolic_volume: PreciseNum, // p=0
    pub simpson_end_systolic_volume: PreciseNum,  // p=0

    pub ascending_aorta_diameter: PreciseNum, // p=1

    pub left_atrium: PreciseNum,        // p=1
    pub left_atrium4: NumXNum,          // p=1
    pub left_atrium_volume: PreciseNum, // p=0

    pub right_atrium_s: PreciseNum,      // p=0
    pub right_atrium4: NumXNum,          // p=1
    pub right_atrium_volume: PreciseNum, // p=0
    pub right_ventricle: PreciseNum,     // p=1
    pub right_ventricle_baz: PreciseNum, // p=1
    // было: right_ventricle_medium_full
    pub right_ventricle_medium: Option<PreciseNum>, // p=1, optional
    // было: right_ventricle_wall_thickness_full
    pub right_ventricle_wall_thickness: Option<PreciseNum>, // p=1, optional
    // было: tapse_full
    pub tapse: Option<PreciseNum>, // p=1, optional
    // было: septum_thickness_baz_full
    pub septum_thickness_baz: Option<PreciseNum>, // p=1, optional

    pub shutters_aortal: ValveLeaflets,
    pub opening_amplitude: PreciseNum, // p=1
    pub max_velocity: PreciseNum,      // p=1 (это max_velocity_aortal)
    pub max_grad: PreciseNum,          // p=0 (это max_grad_aortal)

    // было: mid_grad_full
    pub mid_grad: Option<PreciseNum>, // p=0, only if stenosis=Yes
    // было: s_doppler_full
    pub s_doppler: Option<PreciseNum>, // p=1, only if stenosis=Yes
    // было: s_planim_full
    pub s_planim: Option<PreciseNum>, // p=1, only if stenosis=Yes

    // было: presh_time_full
    pub presh_time: Option<PreciseNum>, // p=0, optional
    // было: vena_contracta_full
    pub vena_contracta: Option<PreciseNum>, // p=1, only if presh_time.is_some()

    // было: max_velocity_vt_full
    pub max_velocity_vt: Option<PreciseNum>, // p=1, optional (only if septum_thickness_baz.is_some())
    // было: max_grad_vt_full
    pub max_grad_vt: Option<PreciseNum>, // p=0, only if max_velocity_vt.is_some()

    pub shutters_mitral: ValveLeaflets,

    pub peak_e: PreciseNum, // p=0
    pub peak_a: PreciseNum, // p=0

    pub tdi_vel: TdiRelation,

    pub e_sept: PreciseNum, // p=0
    pub e_lat: PreciseNum,  // p=0

    // было: max_velocity_mitral_valve_full
    pub max_velocity_mitral_valve: Option<PreciseNum>, // p=1, optional
    // было: max_grad_mitral_valve_full
    pub max_grad_mitral_valve: Option<PreciseNum>, // p=1, depends on max_velocity_mitral_valve
    // было: mid_grad_mitral_valve_full
    pub mid_grad_mitral_valve: Option<PreciseNum>, // p=1, depends on max_velocity_mitral_valve

    pub calts_back_sash: YesNo,
    pub posterior_leaflet_base_calcification: YesNo,

    pub max_velocity_tricuspidal_regurgitation: PreciseNum, // p=1

    pub pulmonary_artery: PreciseNum, // p=1

    pub max_grad_tricuspidal_regurgitation: PreciseNum, // p=0

    // было: pulmonary_artery_right_branch_full
    pub pulmonary_artery_right_branch: Option<PreciseNum>, // p=1, optional
    // было: pulmonary_artery_left_branch_full
    pub pulmonary_artery_left_branch: Option<PreciseNum>, // p=1, only if right_branch.is_some()

    pub max_velocity_in_pulmonary_artery: PreciseNum, // p=1
    pub max_grad_in_pulmonary_artery: PreciseNum,     // p=0

    // было: pulmonary_regurgitation_max_velocity_full
    pub pulmonary_regurgitation_max_velocity: Option<PreciseNum>, // p=1, optional
    // было: pulmonary_regurgitation_max_grad_full
    pub pulmonary_regurgitation_max_grad: Option<PreciseNum>, // p=0, only if max_velocity.is_some()

    pub vena: PreciseNum, // p=1

    pub effusion: PericardialEffusion,

    // --- рассчитываемые значения (внизу структуры) ---
    pub age: i32,

    pub left_atrium_index: f64,
    pub body_surface_area: f64,
    pub ejection_fraction: f64,

    pub left_ventricle_mass: f64,
    pub left_ventricle_mass_index: f64,
    pub relative_wall_thickness: f64,

    pub stroke_volume: AutoValue,

    pub cardiac_output: f64,
    pub cardiac_index: f64,

    pub peak_e_div_peak_a: f64,
    pub e_div_e_aps: f64,

    pub pulmonary_artery_systolic_pressure: f64,
    // было: pulmonary_artery_med_pressure_full
    pub pulmonary_artery_med_pressure: Option<PreciseNum>,

    pub today: DateTime<Local>,
}

impl CalculatedReportData {
    pub fn render(&self) -> EchoReport {
        EchoReport {
            name: self.name.clone(),
            birthday: self.birthday.format("%d.%m.%Y").to_string(),
            department: self.department.to_string(),
            cardnum: self.cardnum.render_to_string(self.today),
            age: self.age.to_string(),

            height: self.height.to_string(),
            weight: self.weight.to_string(),
            pulse: self.pulse.to_string(),

            aortic_sinus_diameter: self.aortic_sinus_diameter.to_string(),
            body_surface_area: PreciseNum::from_float(self.body_surface_area, 2).to_string(),

            left_ventricle_diastolic_size: self.left_ventricle_diastolic_size.to_string(),
            left_ventricle_systolic_size: self.left_ventricle_systolic_size.to_string(),
            septum_thickness: self.septum_thickness.to_string(),
            posterior_wall_thickness: self.posterior_wall_thickness.to_string(),

            left_ventricle_mass: PreciseNum::from_float(self.left_ventricle_mass, 1).to_string(),
            left_ventricle_mass_index: PreciseNum::from_float(self.left_ventricle_mass_index, 1)
                .to_string(),
            relative_wall_thickness: PreciseNum::from_float(self.relative_wall_thickness, 2)
                .to_string(),

            stroke_volume: self.stroke_volume.to_string(),
            cardiac_index: PreciseNum::from_float(self.cardiac_index, 2).to_string(),
            cardiac_output: PreciseNum::from_float(self.cardiac_output, 2).to_string(),

            simpson_end_diastolic_volume: self.simpson_end_diastolic_volume.to_string(),
            simpson_end_systolic_volume: self.simpson_end_systolic_volume.to_string(),
            ejection_fraction: PreciseNum::from_float(self.ejection_fraction, 0).to_string(),

            ascending_aorta_diameter: self.ascending_aorta_diameter.to_string(),

            left_atrium: self.left_atrium.to_string(),
            left_atrium4: self.left_atrium4.to_string(),
            left_atrium_volume: self.left_atrium_volume.to_string(),
            left_atrium_index: PreciseNum::from_float(self.left_atrium_index, 1).to_string(),

            right_atrium_s: self.right_atrium_s.to_string(),
            right_atrium4: self.right_atrium4.to_string(),
            right_atrium_volume: self.right_atrium_volume.to_string(),

            right_ventricle: self.right_ventricle.to_string(),
            right_ventricle_baz: self.right_ventricle_baz.to_string(),
            right_ventricle_medium_full: render_to_string(
                self.right_ventricle_medium,
                ", средний ",
                " см (N< 3,5 см). ",
            ),
            right_ventricle_wall_thickness_full: render_to_string(
                self.right_ventricle_wall_thickness,
                "Толщина передней стенки ПЖ: ",
                " см (N<0,5 см). ",
            ),
            tapse_full: render_to_string(self.tapse, "TAPSE: ", " см (N>=1,7 см)"),
            septum_thickness_baz_full: render_to_string(
                self.septum_thickness_baz,
                "Базальный отдел межжелудочковой перегородки (МЖП): ",
                " см.",
            ),

            shutters_aortal: self.shutters_aortal.render_to_string(),
            opening_amplitude: self.opening_amplitude.to_string(),
            max_velocity: self.max_velocity.to_string(),
            max_grad: self.max_grad.to_string(),

            mid_grad_full: render_to_string(
                self.mid_grad,
                ". Gr ср ",
                " мм рт.ст. (N<20 мм рт.ст.). ",
            ),
            s_doppler_full: render_to_string(
                self.s_doppler,
                "S отверстия АК ",
                " см² (по допплеру)",
            ),
            s_planim_full: render_to_string(self.s_planim, " и ", " см² (планиметрически)"),

            presh_time_full: render_to_string(self.presh_time, "PHT АР ", " мс, VC АР "),
            vena_contracta_full: render_to_string(self.vena_contracta, "", " см."),

            max_velocity_vt_full: render_to_string(
                self.max_velocity_vt,
                "ВТЛЖ: V max - ",
                " м/с (N< 2,0 м/с), ",
            ),
            max_grad_vt_full: render_to_string(self.max_grad_vt, "Gr мах - ", " мм рт.ст."),

            shutters_mitral: self.shutters_mitral.render_to_string(),

            peak_e: self.peak_e.to_string(),
            peak_a: self.peak_a.to_string(),
            peak_e_div_peak_a: PreciseNum::from_float(self.peak_e_div_peak_a, 1).to_string(),

            tdi_vel: self.tdi_vel.to_string(),

            e_sept: self.e_sept.to_string(),
            e_lat: self.e_lat.to_string(),
            e_div_e_aps: PreciseNum::from_float(self.e_div_e_aps, 1).to_string(),

            max_velocity_mitral_valve_full: render_to_string(
                self.max_velocity_mitral_valve,
                "V max  ",
                " м/с (N- 1,1 м/с)",
            ),
            max_grad_mitral_valve_full: render_to_string(
                self.max_grad_mitral_valve,
                ", Gr мах ",
                " мм рт. ст. (N<7 мм рт. ст.)",
            ),
            mid_grad_mitral_valve_full: render_to_string(
                self.mid_grad_mitral_valve,
                ", Gr ср ",
                " мм рт.ст. (N<5 мм рт.ст).",
            ),

            calts_back_sash: self.calts_back_sash.render_to_string(MitVal::Calcinate),
            posterior_leaflet_base_calcification: self
                .posterior_leaflet_base_calcification
                .render_to_string(MitVal::Calcification),

            max_velocity_tricuspidal_regurgitation: self
                .max_velocity_tricuspidal_regurgitation
                .to_string(),

            pulmonary_artery: self.pulmonary_artery.to_string(),
            pulmonary_artery_systolic_pressure: PreciseNum::from_float(
                self.pulmonary_artery_systolic_pressure,
                0,
            )
            .to_string(),
            max_grad_tricuspidal_regurgitation: self.max_grad_tricuspidal_regurgitation.to_string(),

            pulmonary_artery_right_branch_full: render_to_string(
                self.pulmonary_artery_right_branch,
                ", правая ветвь - ",
                " см, ",
            ),
            pulmonary_artery_left_branch_full: render_to_string(
                self.pulmonary_artery_left_branch,
                "левая ветвь - ",
                " см (N<1,5 см)",
            ),

            max_velocity_in_pulmonary_artery: self.max_velocity_in_pulmonary_artery.to_string(),
            max_grad_in_pulmonary_artery: self.max_grad_in_pulmonary_artery.to_string(),

            pulmonary_regurgitation_max_velocity_full: render_to_string(
                self.pulmonary_regurgitation_max_velocity,
                "V max.ЛР ",
                " м/с ",
            ),
            pulmonary_regurgitation_max_grad_full: render_to_string(
                self.pulmonary_regurgitation_max_grad,
                "Макс.град. ЛР ",
                " мм рт.ст.",
            ),
            pulmonary_artery_med_pressure_full: render_to_string(
                self.pulmonary_artery_med_pressure,
                ", Ср.ДЛА ",
                " мм рт.ст. (до 20 мм рт.ст.).",
            ),

            vena: self.vena.to_string(),
            effusion: self.effusion.render_to_string(),

            today: self.today.format("%d.%m.%Y %H:%M").to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EchoReport {
    name: String,
    birthday: String,
    department: String,
    cardnum: String,
    age: String,
    height: String,
    weight: String,
    pulse: String,
    aortic_sinus_diameter: String,
    body_surface_area: String,
    left_ventricle_diastolic_size: String,
    left_ventricle_systolic_size: String,
    septum_thickness: String,
    posterior_wall_thickness: String,
    left_ventricle_mass: String,
    left_ventricle_mass_index: String,
    relative_wall_thickness: String,
    stroke_volume: String,
    cardiac_index: String,
    cardiac_output: String,
    simpson_end_diastolic_volume: String,
    simpson_end_systolic_volume: String,
    ejection_fraction: String,
    ascending_aorta_diameter: String,
    left_atrium: String,
    left_atrium4: String,
    left_atrium_volume: String,
    left_atrium_index: String,
    right_atrium_s: String,
    right_atrium4: String,
    right_atrium_volume: String,
    right_ventricle: String,
    right_ventricle_baz: String,
    right_ventricle_medium_full: String,
    right_ventricle_wall_thickness_full: String,
    tapse_full: String,
    septum_thickness_baz_full: String,
    shutters_aortal: String,
    opening_amplitude: String,
    max_velocity: String,
    max_grad: String,
    mid_grad_full: String,
    s_doppler_full: String,
    s_planim_full: String,
    presh_time_full: String,
    vena_contracta_full: String,
    max_velocity_vt_full: String,
    max_grad_vt_full: String,
    shutters_mitral: String,
    peak_e: String,
    peak_a: String,
    peak_e_div_peak_a: String,
    tdi_vel: String,
    e_sept: String,
    e_lat: String,
    e_div_e_aps: String,
    max_velocity_mitral_valve_full: String,
    max_grad_mitral_valve_full: String,
    mid_grad_mitral_valve_full: String,
    calts_back_sash: String,
    posterior_leaflet_base_calcification: String,
    max_velocity_tricuspidal_regurgitation: String,
    pulmonary_artery: String,
    pulmonary_artery_systolic_pressure: String,
    max_grad_tricuspidal_regurgitation: String,
    pulmonary_artery_right_branch_full: String,
    pulmonary_artery_left_branch_full: String,
    max_velocity_in_pulmonary_artery: String,
    max_grad_in_pulmonary_artery: String,
    pulmonary_regurgitation_max_velocity_full: String,
    pulmonary_regurgitation_max_grad_full: String,
    pulmonary_artery_med_pressure_full: String,
    vena: String,
    effusion: String,
    today: String,
}
