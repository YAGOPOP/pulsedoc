use garbage::input;

// pub struct RawReportData {
//         // Пациент / общие
//     pub patient_name: String,
//     pub birth_date: NaiveDate,
//     pub department: Department,

//     // то, что ты спрашиваешь в зависимости от отделения
//     pub card_number_raw: f64, // AK№ или IB№ как число (у тебя f64, оставляем)
//     // можно хранить сразу готовую строку, но лучше raw + derived cardnum

//     // Антропометрия/ЧСС
//     pub height: f64,
//     pub weight: f64,
//     pub heart_rate: f64,

//     // Аорта
//     pub aortic_sinus_diameter: f64,
//     pub ascending_aorta_diameter: f64,

//     // Левое предсердие
//     pub left_atrium: f64,
//     pub left_atrium4: String,      // "a × b" уже строкой, как у тебя
//     pub left_atrium_volume: f64,

//     // Правое предсердие
//     pub right_atrium4: String,
//     pub right_atrium_s: f64,
//     pub right_atrium_volume: f64,

//     // Правый желудочек
//     pub right_ventricle: f64,
//     pub right_ventricle_baz: f64,
//     pub right_ventricle_medium: Option<f64>,
//     pub right_ventricle_wall_thickness: Option<f64>,
//     pub tapse: Option<f64>,

//     // ЛЖ
//     pub left_ventricle_diastolic_size: f64,
//     pub left_ventricle_systolic_size: f64,
//     pub septum_thickness: f64,
//     pub septum_thickness_baz: Option<f64>,
//     pub posterior_wall_thickness: f64,

//     // Simpson
//     pub simpson_end_diastolic_volume: f64,
//     pub simpson_end_systolic_volume: f64,
//     pub stroke_volume_manual: Option<f64>, // "УО (или Enter)" — то, что вводит пользователь

//     // АК (описание створок + доп. параметры)
//     pub shutters_aortal: ValveLeaflets,
//     pub opening_amplitude: f64,
//     pub max_velocity_aortal: f64,
//     pub max_grad_aortal: f64,

//     // стеноз АК (условно)
//     pub aortal_stenosis_present: bool,
//     pub mid_grad_aortal: Option<f64>,
//     pub s_doppler: Option<f64>,
//     pub s_planim: Option<f64>,

//     // АР / ВТЛЖ / МК
//     pub presh_time: Option<f64>,
//     pub max_velocity_vt: Option<f64>, // зависит от septum_thickness_baz
//     pub shutters_mitral: ValveLeaflets,

//     pub calts_back_sash: bool,
//     pub posterior_leaflet_base_calcification: bool,

//     // диастолика
//     pub peak_e: f64,
//     pub peak_a: f64,
//     pub tdi_vel: String, // "e<a" / "e>a"
//     pub e_sept: f64,
//     pub e_lat: f64,

//     // МК скорости (опционально)
//     pub max_velocity_mitral_valve: Option<f64>,

//     // ТК / давление
//     pub max_velocity_tricuspidal_regurgitation: f64,
//     pub max_grad_tricuspidal_regurgitation: f64,
//     pub right_atrium_pressure: f64, // 3 или "иное"

//     // ЛА
//     pub pulmonary_artery: f64,
//     pub pulmonary_artery_right_branch: Option<f64>, // левая берётся “depends_of”
//     pub max_velocity_in_pulmonary_artery: f64,
//     pub max_grad_in_pulmonary_artery: f64,

//     pub pulmonary_regurgitation_max_velocity: Option<f64>, // ЛР
//     // НПВ
//     pub vena: f64,

//     // Выпот
//     pub effusion: String,
// }

// impl RawReportData {
//     pub fn gather() -> RawReportData {
        
//     }
// }

fn ask_loop<T>(
    msg: &str, 
    mut parse: impl FmMut(String) -> Result<T, String>
) -> T {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => continue,
        };

        match parse(inp) {
            Ok(v) => return v,
            Err(e) => {
                eprintln("Ошибка: {}", e);
                continue;
            },
        }
    }
}