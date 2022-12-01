use app::{get_branches, get_log, get_log_tree, get_status, get_tree};
use clap::{App, Arg};
use stats::get_stats;
use std::io;
use tui::style::Color;

mod app;
mod ui;

mod stats;
use crate::stats::get_percentages;

mod terminal;
use crate::terminal::setup_terminal;
use crate::terminal::TApp;

fn main() -> Result<(), io::Error> {
    let matches = App::new("Pstat")
        .version("0.1.0")
        .author("SP457")
        .about("Project Statistics TUI")
        .arg(
            Arg::with_name("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .help("Path to project directory (Defaults to current path)"),
        )
        .arg(
            Arg::with_name("ignore")
                .short('i')
                .long("ignore")
                .help("Use .gitignore if exists"),
        )
        .arg(
            Arg::with_name("color")
                .short('c')
                .long("color")
                .takes_value(true)
                .help("Color scheme to use (Refer to colors supported by tui::style::Color)"),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");
    let color = matches.value_of("color").unwrap_or("LightBlue");

    let app_color = match color {
        "Black" => Color::Black,
        "Red" => Color::Red,
        "Green" => Color::Green,
        "Yellow" => Color::Yellow,
        "Blue" => Color::Blue,
        "Magenta" => Color::Magenta,
        "Cyan" => Color::Cyan,
        "Gray" => Color::Gray,
        "DarkGray" => Color::DarkGray,
        "LightRed" => Color::LightRed,
        "LightGreen" => Color::LightGreen,
        "LightYellow" => Color::LightYellow,
        "LightBlue" => Color::LightBlue,
        "LightMagenta" => Color::LightMagenta,
        "LightCyan" => Color::LightCyan,
        "White" => Color::White,
        _ => Color::White,
    };

    let mut ignore = matches.occurrences_of("ignore") > 0;

    if ignore && !(path.eq(".") || path.eq("./")) {
        println!("Cannot use -i with another directory other than the current one.");
        println!("Either use the current working directory or omit the -i flag.");
        return Ok(());
    }

    let (file_stats, proj_size, times) = get_stats(path, &mut ignore);
    let lang_stats = get_percentages(&file_stats, proj_size);

    let mut count_time: Vec<(&String, &u64)> = times.iter().collect();
    count_time.sort_by(|a, b| a.1.cmp(b.1));

    let mut file_time: Vec<String> = Vec::new();
    for i in count_time.iter().take(5) {
        file_time.push(i.0.to_string());
    }

    let branches = get_branches(path);
    let log_tree = get_log_tree(path);
    let log = get_log(path);
    let status = get_status(path);
    let tree = get_tree(ignore, path);

    let mut app = TApp {
        scroll: (0, 0),
        status_scroll: (0, 0),
        tree,
        path: String::from(path),
        file_stats,
        lang_stats,
        branches,
        log,
        log_tree,
        status,
        file_time,
        app_color,
        tab: 0,
        verbose: false,
    };

    setup_terminal(&mut app)
}
