use std::env;
use std::io;
use std::process::Command;

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

    let mut app = App {
        scroll: (0, 0),
        tree: output,
        path: String::from(path),
        file_stats: file_stats,
        lang_stats: lang_stats,
    };

    setup_terminal(&mut app)
}
