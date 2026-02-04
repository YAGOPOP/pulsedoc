use chrono::{DateTime, Datelike, Local, NaiveDate};
use inquire::{InquireError, Select, Text};
use std::{fmt, process};

// типы начало
#[derive(Debug, Clone, Copy)]
pub struct PreciseNum {
    value: f64,
    precision: u8,
}

#[derive(Debug, Clone)]
pub struct AutoValue {
    pub value: PreciseNum,
    pub auto: bool,
}

impl fmt::Display for AutoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = if self.auto {
            ""
        } else {
            " (по допплеру)"
        };
        write!(f, "{} мл{}", self.value, v)
    }
}

pub trait RenderToString {
    fn render_to_string(&self) -> String;
}

impl fmt::Display for PreciseNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:.*}", self.precision as usize, self.value).replace('.', ",");
        write!(f, "{s}")
    }
}

impl PreciseNum {
    pub fn value(self) -> f64 {
        self.value
    }

    pub fn new_scaled(n: i64, p: u8) -> Self {
        Self {
            value: n as f64 / 10_f64.powi(p as i32),
            precision: p,
        }
    }
    pub fn from_float(n: f64, p: u8) -> Self {
        Self {
            value: n,
            precision: p,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    InvalidDate,
    InvalidNumber,
    EmptyNumber,
    NumXNumNeedTwo,
    NumXNumFirstInvalid,
    NumXNumSecondInvalid,
    NumXNumTooMany,
    EmptyString,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ParseError::InvalidDate => "Некорректная дата (ожидаю ДДММГГГГ).",
            ParseError::InvalidNumber => "Некорректное число.",
            ParseError::EmptyNumber => "Ввод этого числа нельзя пропустить.",
            ParseError::NumXNumNeedTwo => "Введите два числа через пробел.",
            ParseError::NumXNumFirstInvalid => "Первое число некорректно.",
            ParseError::NumXNumSecondInvalid => "Второе число некорректно.",
            ParseError::NumXNumTooMany => "Должно быть ровно два числа, введено более двух.",
            ParseError::EmptyString => "Строка не должна быть пустой.",
        };
        write!(f, "{msg}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NumXNum {
    num1: PreciseNum,
    num2: PreciseNum,
}

impl fmt::Display for NumXNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.num1, self.num2)
    }
}
// типы конец

// мелочь начало
pub fn calc_age(birthday: NaiveDate, td: DateTime<Local>) -> i32 {
    let today = td.date_naive();
    let mut age: i32 = today.year() - birthday.year();
    if today.ordinal() < birthday.ordinal() {
        age -= 1;
    }
    age
}

fn prep_num_msg(msg: &str, p: u8) -> String {
    if p == 0 {
        format!("{} (целое)", msg)
    } else {
        format!("{} (/{})", msg, 10_i64.pow(p as u32))
    }
}

pub fn render_to_string(value: Option<PreciseNum>, left: &str, right: &str) -> String {
    match value {
        Some(v) => format!("{}{}{}", left, v, right),
        None => "".to_owned(),
    }
}
// мелочь конец

// ядерные ф-ции начало
fn input(msg: &str) -> Option<String> {
    let msg = format!("{}:", msg);
    let inp = match Text::new(&msg).prompt() {
        Ok(i) => i,
        Err(InquireError::OperationCanceled) => {
            eprintln!("Ввод отменён.");
            return None;
        }
        Err(InquireError::OperationInterrupted) => {
            eprintln!("\nВыполнение прервано.");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Ошибка ввода: {}", e);
            return None;
        }
    };
    Some(inp.trim().to_owned())
}

pub fn ask_required<T>(msg: &str, mut parse: impl FnMut(&str) -> Result<T, ParseError>) -> T {
    loop {
        let inp = match input(msg) {
            Some(v) => v,
            None => continue,
        };

        match parse(&inp) {
            Ok(v) => return v,
            Err(e) => {
                eprintln!("Ошибка: {}", e);
                continue;
            }
        }
    }
}

pub fn ask_optional<T>(
    msg: &str,
    mut parse: impl FnMut(&str) -> Result<T, ParseError>,
) -> Option<T> {
    loop {
        let inp = match input(&format!("{} (или нажмите Ввод чтобы пропустить)", msg))
        {
            Some(v) => v,
            None => continue,
        };

        if inp.is_empty() {
            return None;
        }

        match parse(&inp) {
            Ok(v) => return Some(v),
            Err(e) => eprintln!("Ошибка: {}", e),
        }
    }
}

pub fn ask_selection<T: fmt::Display + strum::IntoEnumIterator + Clone>(msg: &str) -> T {
    loop {
        let options: Vec<T> = T::iter().collect();
        match Select::new(&format!("{}:", msg), options).prompt() {
            Ok(v) => return v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}
//ядерные ф-ции конец

// парсеры начало
pub fn parse_string(inp: &str) -> Result<String, ParseError> {
    if inp.is_empty() {
        Err(ParseError::EmptyString)
    } else {
        Ok(inp.to_owned())
    }
}

pub fn parse_date(inp: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(inp, "%d%m%Y").map_err(|_| ParseError::InvalidDate)
}

pub fn parse_num_precise(inp: &str, precision: u8) -> Result<PreciseNum, ParseError> {
    if inp.is_empty() {
        Err(ParseError::EmptyNumber)
    } else {
        let num: i64 = inp.parse().map_err(|_| ParseError::InvalidNumber)?;
        Ok(PreciseNum::new_scaled(num, precision))
    }
}

pub fn parse_num_x_num(inp: &str, precision: u8) -> Result<NumXNum, ParseError> {
    let mut parts = inp.split_whitespace();

    let num1 = parts.next().ok_or(ParseError::NumXNumNeedTwo)?;
    let num1 = parse_num_precise(num1, precision).map_err(|_| ParseError::NumXNumFirstInvalid)?;

    let num2 = parts.next().ok_or(ParseError::NumXNumNeedTwo)?;
    let num2 = parse_num_precise(num2, precision).map_err(|_| ParseError::NumXNumSecondInvalid)?;

    if parts.next().is_some() {
        return Err(ParseError::NumXNumTooMany);
    }

    return Ok(NumXNum { num1, num2 });
}

pub fn parse_int(inp: &str) -> Result<i64, ParseError> {
    if inp.is_empty() {
        Err(ParseError::EmptyNumber) // или отдельный EmptyInt
    } else {
        inp.parse().map_err(|_| ParseError::InvalidNumber)
    }
}

// парсеры конец

// обёртки-геттеры начало
pub fn get_int(msg: &str) -> i64 {
    ask_required(msg, parse_int)
}

pub fn get_int_if(cond: bool, msg: &str) -> Option<i64> {
    return if cond { Some(get_int(msg)) } else { None };
}

pub fn get_string(msg: &str) -> String {
    ask_required(msg, parse_string)
}

pub fn get_date(msg: &str) -> NaiveDate {
    ask_required(&format!("{} (ДДММГГГГ)", msg), parse_date)
}

pub fn get_num(msg: &str, p: u8) -> PreciseNum {
    ask_required(&prep_num_msg(msg, p), |s| parse_num_precise(s, p))
}

pub fn get_num_if(cond: bool, msg: &str, p: u8) -> Option<PreciseNum> {
    return if cond { Some(get_num(msg, p)) } else { None };
}

pub fn get_num_opt(msg: &str, p: u8) -> Option<PreciseNum> {
    ask_optional(&prep_num_msg(msg, p), |s| parse_num_precise(s, p))
}

pub fn get_num_opt_if(cond: bool, msg: &str, p: u8) -> Option<PreciseNum> {
    return if cond { get_num_opt(msg, p) } else { None };
}

pub fn get_num_x_num(msg: &str, p: u8) -> NumXNum {
    ask_required(
        &format!("{} {}", prep_num_msg(msg, p), "(2 числа через пробел)"),
        |s| parse_num_x_num(s, p),
    )
}
// обёртки-геттеры конец
