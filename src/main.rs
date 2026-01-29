use chrono::{DateTime, Local, NaiveDate};
use serde_json::Value;
mod garbage;
use garbage::{
    EchoReport, StrokeVolume, calc_age, get_selected, load_settings, prep_num, prep_num_opt,
    prep_num_precise, prep_volume, safe_get_date, safe_get_num, safe_get_num_opt,
    safe_get_num_x_num, safe_get_string, simple_num_depends_of, simple_num_depends_of_opt,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let today: DateTime<Local> = Local::now();

    let klapan = vec![
        " без особенностей",
        " уплотнены",
        " уплотнены, с включением мелких кальцинатов",
        " уплотнены, кальцинированы",
    ];

    let cur_settings = load_settings();

    let name: String = safe_get_string("ФИО:");
    let birthday: NaiveDate = safe_get_date("Дата рождения (ДДММГГГГ):");

    let department = get_selected("Отделение:", vec!["КДО", "ДиОТ", "пульм", "ревм", "КХО"]);

    let cardnum: String = match &department as &str {
        "КДО" => {
            let aknum: f64 = safe_get_num("АК№:", 0);
            format!("АК№: {}-{}-А", aknum, today.format("%Y"))
        }
        _ => {
            let ibnum: f64 = safe_get_num("ИБ№:", 0);
            format!("ИБ№: {}-{}-C", ibnum, today.format("%y"))
        }
    };

    let height: f64 = safe_get_num("Рост:", 0);
    let weight: f64 = safe_get_num("Вес:", 0);
    let pulse: f64 = safe_get_num("ЧСС:", 0);
    let aortic_sinus_diameter: f64 = safe_get_num("Ао (*10^(-1)):", 1);
    let ascending_aorta_diameter: f64 = safe_get_num("ВА (*10^(-1)):", 1);
    let left_atrium: f64 = safe_get_num("ЛП:", 1);
    let left_atrium4: String = safe_get_num_x_num("ЛП4 (2 числа (*10^(-1)) через пробел):", 1);
    let left_atrium_volume: f64 = safe_get_num("ЛП V:", 0);
    let right_atrium4: String = safe_get_num_x_num("ПП4 (2 числа (*10^(-1)) через пробел):", 1);
    let right_atrium_s: f64 = safe_get_num("ПП S:", 0);
    let right_atrium_volume: f64 = safe_get_num("ПП V:", 0);
    let right_ventricle: f64 = safe_get_num("ПЗР ПЖ (*10^(-1)):", 1);
    let right_ventricle_baz: f64 = safe_get_num("ПЖ баз (*10^(-1)):", 1);

    let right_ventricle_medium: Option<f64> =
        safe_get_num_opt("ПЖ ср (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);
    let right_ventricle_wall_thickness: Option<f64> =
        safe_get_num_opt("ПСПЖ (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);
    let tapse: Option<f64> =
        safe_get_num_opt("TAPSE (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);

    let left_ventricle_diastolic_size: f64 = safe_get_num("КДР (*10^(-1)):", 1);
    let left_ventricle_systolic_size: f64 = safe_get_num("КСР (*10^(-1)):", 1);
    let septum_thickness: f64 = safe_get_num("МЖП (*10^(-1)):", 1);
    let septum_thickness_baz: Option<f64> =
        safe_get_num_opt("МЖП баз (*10^(-1) или нажмите Ввод чтобы пропустить):", 1);
    let posterior_wall_thickness: f64 = safe_get_num("ЗС (*10^(-1)):", 1);

    let simpson_end_diastolic_volume: f64 = safe_get_num("КДО (по Симпсону):", 0);
    let simpson_end_systolic_volume: f64 = safe_get_num("КСО (по Симпсону):", 0);
    let stroke_volume: Option<f64> = safe_get_num_opt("УО (или нажмите Ввод чтобы пропустить)", 0);
    let shutters_aortal: String = format!("{}.", get_selected("АК:", klapan.clone()));

    let opening_amplitude: f64 = safe_get_num("Амплитуда раскрытия (*10^(-1)):", 1);
    let max_velocity: f64 = safe_get_num("Макс скорость (*10^(-1)):", 1);
    let max_grad: f64 = safe_get_num("Макс градиент:", 0);

    let mut mid_grad: Option<f64> = None;
    let mut s_doppler: Option<f64> = None;
    let mut s_planim: Option<f64> = None;
    match &get_selected("Стеноз:", vec!["нет", "есть"]) as &str {
        "есть" => {
            mid_grad = Some(safe_get_num("Средний градиент:", 0));
            s_doppler = Some(safe_get_num("Площадь по допплеру (*10^(-1)):", 1));
            s_planim = Some(safe_get_num("Площадь планиметрически (*10^(-1)):", 1));
        }
        _ => {}
    }

    let presh_time: Option<f64> = safe_get_num_opt("PHT (или нажмите Ввод чтобы пропустить):", 0);
    let vena_contracta: Option<f64> = simple_num_depends_of(presh_time, "VC АР (*10^(-1)):", 1);
    let max_velocity_vt = simple_num_depends_of_opt(
        septum_thickness_baz,
        "ВТЛЖ Макс скорость (*10^(-1) или нажмите Ввод чтобы пропустить):",
        1,
    );

    let max_grad_vt = simple_num_depends_of(max_velocity_vt, "ВТЛЖ макс градиент:", 0);
    let shutters_mitral: String = format!("{}. ", get_selected("МК:", klapan));

    let yes_no = vec!["Нет", "Да"];
    let mut calts_back_sash = "Кальцинат в основании задней створки:".to_owned();
    match &get_selected(&calts_back_sash, yes_no.clone()) as &str {
        "Да" => calts_back_sash = "Кальцинат в основании задней створки. ".to_owned(),
        _ => calts_back_sash = String::new(),
    }

    let mut posterior_leaflet_base_calcification =
        "Кальциноз основания задней створки, фиброзного кольца".to_owned();
    match &get_selected(&posterior_leaflet_base_calcification, yes_no.clone()) as &str {
        "Да" => {
            posterior_leaflet_base_calcification =
                format!("{}. ", posterior_leaflet_base_calcification)
        }
        _ => posterior_leaflet_base_calcification = String::new(),
    }

    let peak_e: f64 = safe_get_num("МК: Е:", 0);
    let peak_a: f64 = safe_get_num("А:", 0);
    let tdi_vel: String = get_selected("TDI:", vec!["e<a", "e>a"]);
    let e_sept: f64 = safe_get_num("E sept:", 0);
    let e_lat: f64 = safe_get_num("E’ lat:", 0);
    let max_velocity_mitral_valve = safe_get_num_opt(
        "МК Макс скорость (*10^(-1) или нажмите Ввод чтобы пропустить):",
        1,
    );

    let max_grad_mitral_valve: Option<f64> =
        simple_num_depends_of(max_velocity_mitral_valve, "МК Макс градиент:", 1);
    let mid_grad_mitral_valve: Option<f64> =
        simple_num_depends_of(max_velocity_mitral_valve, "МК Средний градиент:", 1);

    let max_velocity_tricuspidal_regurgitation: f64 =
        safe_get_num("ТК Макс скорость ТР (*10^(-1)):", 1);
    let max_grad_tricuspidal_regurgitation: f64 = safe_get_num("ТК макс градиент ТР:", 0);

    let my_message = format!(
        "СДЛА: к {} прибавить 3?",
        max_grad_tricuspidal_regurgitation
    );

    let yes_other = vec!["да", "иное"];
    let right_atrium_pressure: f64 = match &get_selected(&my_message as &str, yes_other) as &str {
        "да" => 3.0,
        _ => safe_get_num("Иное:", 0),
    };

    let pulmonary_artery: f64 = safe_get_num("Диаметр ЛА (*10^(-1)):", 1);

    let pulmonary_artery_right_branch = safe_get_num_opt(
        "Правая ветвь ЛА (*10^(-1) или нажмите Ввод чтобы пропустить):",
        1,
    );
    let pulmonary_artery_left_branch = simple_num_depends_of(
        pulmonary_artery_right_branch,
        "Левая ветвь ЛА (*10^(-1)): ",
        1,
    );

    let max_velocity_in_pulmonary_artery = safe_get_num("ЛА макс скорость (*10^(-1)):", 1);
    let max_grad_in_pulmonary_artery = safe_get_num("ЛА макс градиент:", 0);
    let pulmonary_regurgitation_max_velocity = safe_get_num_opt(
        "ЛР макс. скорость (*10^(-1)  или нажмите Ввод чтобы пропустить):",
        1,
    );
    let pulmonary_regurgitation_max_grad =
        simple_num_depends_of(pulmonary_regurgitation_max_velocity, "ЛР макс градиент:", 0);

    let vena: f64 = safe_get_num("НПВ (*10^(-1)):", 1);

    let vypot: Vec<&str> = vec![
        "не выявлен.",
        "эхонегативное пространство по задней стенке левого желудочка до см, по боковой стенке левого желудочка до см, по боковой стенке правого желудочка см, по передней стенке правого желудочка см. по верхне-латеральному краю правого предсердия до см.",
    ];
    let effusion: String = get_selected("Перикардиальный выпот: ", vypot);

    //расчёты начало
    let body_surface_area: f64 =
        f64::powf(height as f64, 0.725) * f64::powf(weight as f64, 0.425) * 0.007;

    let left_atrium_index: f64 = left_atrium_volume as f64 / body_surface_area;

    let age = calc_age(birthday, today);

    let ejection_fraction = (simpson_end_diastolic_volume - simpson_end_systolic_volume)
        / simpson_end_diastolic_volume
        * 100.0;

    let left_ventricle_mass: f64 = 0.8
        * (1.04
            * ((left_ventricle_diastolic_size + septum_thickness + posterior_wall_thickness)
                .powi(3)
                - left_ventricle_diastolic_size.powi(3)))
        + 0.6;

    let left_ventricle_mass_index: f64 = left_ventricle_mass / body_surface_area;

    let relative_wall_thickness: f64 =
        2.0 * posterior_wall_thickness / left_ventricle_diastolic_size;

    // если вручную то по допплеру
    let stroke_volume = match stroke_volume {
        None => StrokeVolume {
            value: simpson_end_diastolic_volume - simpson_end_systolic_volume,
            auto: true,
        },
        Some(v) => StrokeVolume {
            value: v,
            auto: false,
        },
    };

    let cardiac_output: f64 = pulse * stroke_volume.value / 1000.0;

    let cardiac_index: f64 = cardiac_output / body_surface_area;

    let peak_e_div_peak_a: f64 = peak_e / peak_a;

    let e_div_e_aps: f64 = peak_e / ((e_sept + e_lat) / 2.0);

    let pulmonary_artery_systolic_pressure: f64 =
        max_grad_tricuspidal_regurgitation + right_atrium_pressure;

    let pulmonary_artery_med_pressure: Option<f64> = match pulmonary_regurgitation_max_grad {
        Some(v) => Some(v + right_atrium_pressure),
        None => None,
    };
    //расчёты конец

    // имя файла
    let out_filename: String = format!("{} {}.docx", &name, today.format("%y%m%d"));
    // имя файла

    let ready_data = EchoReport {
        name,
        birthday: birthday.format("%d.%m.%Y").to_string(),
        department,
        aortic_sinus_diameter: prep_num_precise(aortic_sinus_diameter, 1),
        ascending_aorta_diameter: prep_num_precise(ascending_aorta_diameter, 1),
        cardnum,
        cardiac_index: prep_num_precise(cardiac_index, 1),
        cardiac_output: prep_num_precise(cardiac_output, 1),
        ejection_fraction: prep_num_precise(ejection_fraction, 0),
        height: prep_num(height),
        left_atrium: prep_num_precise(left_atrium, 1),
        left_atrium4,
        left_atrium_volume: prep_num(left_atrium_volume),
        left_ventricle_diastolic_size: prep_num_precise(left_ventricle_diastolic_size, 1),
        left_ventricle_mass: prep_num_precise(left_ventricle_mass, 1),
        left_ventricle_mass_index: prep_num_precise(left_ventricle_mass_index, 2),
        left_ventricle_systolic_size: prep_num_precise(left_ventricle_systolic_size, 1),
        posterior_wall_thickness: prep_num_precise(posterior_wall_thickness, 1),
        pulse: prep_num(pulse),
        relative_wall_thickness: prep_num_precise(relative_wall_thickness, 2),
        right_atrium4,
        right_atrium_s: prep_num(right_atrium_s),
        right_atrium_volume: prep_num(right_atrium_volume),
        right_ventricle: prep_num_precise(right_ventricle, 1),
        right_ventricle_baz: prep_num_precise(right_ventricle_baz, 1),
        septum_thickness: prep_num_precise(septum_thickness, 1),
        septum_thickness_baz_full: prep_num_opt(
            septum_thickness_baz,
            "Базальный отдел межжелудочковой перегородки (МЖП): ",
            " см.",
            1,
        ),
        simpson_end_diastolic_volume: prep_num(simpson_end_diastolic_volume),
        simpson_end_systolic_volume: prep_num(simpson_end_systolic_volume),
        stroke_volume: prep_volume(stroke_volume, 0),
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

        shutters_aortal,
        opening_amplitude: prep_num_precise(opening_amplitude, 1),
        max_velocity: prep_num_precise(max_velocity, 1),
        max_grad: prep_num(max_grad),
        shutters_mitral,
        peak_e: prep_num(peak_e),
        peak_a: prep_num(peak_a),
        peak_e_div_peak_a: prep_num_precise(peak_e_div_peak_a, 1),

        mid_grad_full: prep_num_opt(mid_grad, ". Gr ср ", " мм рт.ст", 0),
        s_doppler_full: prep_num_opt(
            s_doppler,
            ". (N<20 мм рт.ст.). S отверстия АК ",
            " см² (по допплеру) и ",
            1,
        ),
        s_planim_full: prep_num_opt(s_planim, "", " см² (планиметрически)", 1),
        presh_time_full: prep_num_opt(presh_time, "PHT АР ", " мс, VC АР ", 0),
        vena_contracta_full: prep_num_opt(vena_contracta, "", " см.", 1),

        calts_back_sash,
        posterior_leaflet_base_calcification,

        max_velocity_vt_full: prep_num_opt(
            max_velocity_vt,
            "ВТЛЖ: V max - ",
            " м/с (N< 2,0 м/с), Gr мах - ",
            1,
        ),
        max_grad_vt_full: prep_num_opt(max_grad_vt, "", " мм рт.ст.", 0),
        tdi_vel,
        e_sept: prep_num(e_sept),
        e_lat: prep_num(e_lat),
        e_div_e_aps: prep_num_precise(e_div_e_aps, 0),

        max_velocity_mitral_valve_full: prep_num_opt(
            max_velocity_mitral_valve,
            "V max  ",
            " м/с (N- 1,1 м/с), Gr мах ",
            1,
        ),
        max_grad_mitral_valve_full: prep_num_opt(
            max_grad_mitral_valve,
            "",
            " мм рт. ст. (N<7 мм рт. ст.), Gr ср ",
            1,
        ),
        mid_grad_mitral_valve_full: prep_num_opt(
            mid_grad_mitral_valve,
            "",
            " мм рт.ст. (N<5 мм рт.ст).",
            1,
        ),

        tapse_full: prep_num_opt(tapse, ". TAPSE: ", " см (N>=1,7 см)", 1),
        age: prep_num(age),
        body_surface_area: prep_num_precise(body_surface_area, 2),
        left_atrium_index: prep_num_precise(left_atrium_index, 1),
        today: today.format("%d.%m.%Y").to_string(),

        max_velocity_tricuspidal_regurgitation: prep_num(max_velocity_tricuspidal_regurgitation),
        pulmonary_artery: prep_num_precise(pulmonary_artery, 1),
        pulmonary_artery_systolic_pressure: prep_num(pulmonary_artery_systolic_pressure),
        max_grad_tricuspidal_regurgitation: prep_num(max_grad_tricuspidal_regurgitation),

        pulmonary_artery_right_branch_full: prep_num_opt(
            pulmonary_artery_right_branch,
            ", правая ветвь - ",
            " см, левая ветвь - ",
            1,
        ),
        pulmonary_artery_left_branch_full: prep_num_opt(
            pulmonary_artery_left_branch,
            "",
            " см (N<1,5 см)",
            1,
        ),
        max_grad_in_pulmonary_artery: prep_num(max_grad_in_pulmonary_artery),
        max_velocity_in_pulmonary_artery: prep_num_precise(max_velocity_in_pulmonary_artery, 1),
        pulmonary_regurgitation_max_velocity_full: prep_num_opt(
            pulmonary_regurgitation_max_velocity,
            "V max.ЛР ",
            " м/с ",
            1,
        ),
        pulmonary_regurgitation_max_grad_full: prep_num_opt(
            pulmonary_regurgitation_max_grad,
            "Макс.град. ЛР ",
            " мм рт.ст.",
            0,
        ),

        pulmonary_artery_med_pressure_full: prep_num_opt(
            pulmonary_artery_med_pressure,
            ", Ср.ДЛА ",
            " мм рт.ст. (до 20 мм рт.ст.).",
            0,
        ),

        vena: prep_num_precise(vena, 1),
        effusion,
    };

    // // работа с файлами начало

    let template_bytes = fs::read("./assets/tplt.docx")?;

    let data: Value = serde_json::to_value(&ready_data)?;

    let rendered_bytes = docx_handlebars::render_template(template_bytes, &data)?;

    let my_path = cur_settings.save_dir;
    fs::write(my_path.join(out_filename), rendered_bytes)?;

    return Ok(());
}
