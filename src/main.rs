use std::env;
use std::io;
use std::process::Command;
mod ui;

mod stats;
use crate::stats::{get_percentages, list_files};

mod terminal;
use crate::terminal::setup_terminal;
use crate::terminal::App;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => path,
        None => ".",
    };

    let (file_stats, proj_size) = list_files(path);
    let lang_stats = get_percentages(&file_stats, proj_size);

    let output = Command::new("tree")
        .arg(path)
        .output()
        .expect("Tree command failed");
    let output = String::from_utf8(output.stdout).unwrap();

    let branches = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("branch")
        .output()
        .expect("git branch command failed");
    let branches = String::from_utf8(branches.stdout).unwrap();

    let log = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("-n 5")
        .output()
        .expect("git log command failed");
    let log = String::from_utf8(log.stdout).unwrap();

    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .output()
        .expect("git status command failed");
    let status = String::from_utf8(status.stdout).unwrap();

    let mut app = App {
        scroll: (0, 0),
        tree: output,
        path: String::from(path),
        file_stats,
        lang_stats,
        branches,
        log,
        status,
        tab: 0,
    };

    setup_terminal(&mut app)
}
