use anstyle::{Effects, RgbColor, Style};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Color, Table};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub const BAT_BLUE: Style = Style::new()
    .fg_color(Some(anstyle::Color::Rgb(RgbColor(65, 145, 255))))
    .effects(Effects::BOLD);

pub const STEEL_BLUE: Style = Style::new()
    .fg_color(Some(anstyle::Color::Rgb(RgbColor(140, 185, 235))))
    .effects(Effects::BOLD);

pub const GOTHAM_RED: Style = Style::new()
    .fg_color(Some(anstyle::Color::Rgb(RgbColor(255, 75, 75))))
    .effects(Effects::BOLD);

pub const ARCTIC_MINT: Style = Style::new()
    .fg_color(Some(anstyle::Color::Rgb(RgbColor(90, 230, 200))))
    .effects(Effects::BOLD);

pub fn log_step(step: &str, details: &str) {
    anstream::println!("{BAT_BLUE}[{step}]{BAT_BLUE:#} {details}");
}

pub fn log_success(target: &str, message: &str) {
    anstream::println!("  {ARCTIC_MINT}✔{ARCTIC_MINT:#} {STEEL_BLUE}{target}{STEEL_BLUE:#}: {message}");
}

pub fn log_error(err: &str) {
    anstream::eprintln!("{GOTHAM_RED}[error]{GOTHAM_RED:#} {err}");
}

pub fn spinner(message: &'static str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{prefix:.bold.256color(69)} {spinner:.256color(33)} {msg}")
            .unwrap(),
    );
    pb.set_prefix("[sertemp]");
    pb.set_message(message);
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn create_table() -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    table
}

pub fn bat_header_color() -> Color {
    Color::Rgb {
        r: 65,
        g: 145,
        b: 255,
    }
}

pub fn steel_cell_color() -> Color {
    Color::Rgb {
        r: 140,
        g: 185,
        b: 235,
    }
}

pub fn mint_cell_color() -> Color {
    Color::Rgb {
        r: 90,
        g: 230,
        b: 200,
    }
}
