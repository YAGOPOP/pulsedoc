mod promptget;
mod report;
mod settings;
use crate::{report::CalculatedReportData, settings::get_exe_dir};
use chrono::{DateTime, Local};
use report::RawReportData;
use serde_json::Value;
use settings::load_settings;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cur_settings = load_settings();
    let today: DateTime<Local> = Local::now();

    let raw_report = RawReportData::gather();
    let calculated_report = CalculatedReportData::from_raw(&raw_report, today);
    let rendered_report = calculated_report.render();

    let out_filename: String = format!(
        "{} {}.docx",
        &calculated_report.name,
        today.format("%y%m%d")
    );

    let tplt_loc = get_exe_dir().join("assets").join("tplt.docx");
    let data: Value = serde_json::to_value(&rendered_report)?;

    let template_bytes = fs::read(tplt_loc)?;
    let rendered_bytes = docx_handlebars::render_template(template_bytes, &data)?;

    let save_dir = cur_settings.get_save_dir();
    let _ = fs::create_dir(&save_dir);
    fs::write(save_dir.join(out_filename), rendered_bytes)?;

    Ok(())
}
