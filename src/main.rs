use std::io;
use std::path::Path;
use clap::{App, Arg};
use std::process::{Command, Stdio};
mod ui;

mod stats;
use crate::stats::{get_percentages, list_files};

mod terminal;
use crate::terminal::setup_terminal;
use crate::terminal::TApp;

fn get_tree(ignore: bool, path: &str) -> String {
    if ignore && Path::new("./.gitignore").exists() {
        let git_ls = Command::new("git")
            .arg("ls-tree")
            .arg("-r")                
            .arg("--name-only")                
            .arg("HEAD")                
            .stdout(Stdio::piped())     
            .spawn()                      
            .unwrap();            
                 
        let tree = Command::new("tree")
                .arg("--fromfile")
                .stdin(Stdio::from(git_ls.stdout.unwrap())) 
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

        return String::from_utf8(tree.wait_with_output().unwrap().stdout).unwrap();
    } else {
        let output1 = Command::new("tree")
            .arg(path)
            .output()
            .expect("Tree command failed");
        return String::from_utf8(output1.stdout).unwrap();
    }
}

fn main() -> Result<(), io::Error> {
    let matches = App::new("Pstat")
        .version("0.1.0")
        .author("SP457")
        .about("Project Statistics TUI")
        .arg(Arg::with_name("path")
                 .short('p')
                 .long("path")
                 .takes_value(true)
                 .help("Path to project directory"))
        .arg(Arg::with_name("ignore")
                 .short('i')
                 .long("ignore")
                 .help("Use .gitignore if exists"))
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");
    let mut ignore = matches.occurrences_of("ignore") > 0;
   
    let (file_stats, proj_size, times) = list_files(path, &mut ignore);
    let lang_stats = get_percentages(&file_stats, proj_size);

    let mut count_time: Vec<(&String, &u64)> = times.iter().collect();
    count_time.sort_by(|a, b| b.1.cmp(a.1));

    let mut file_time: Vec<String> = Vec::new();
    for i in count_time.iter().take(5) {
        file_time.push(i.0.to_string());
    }


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

    let mut app = TApp {
        scroll: (0, 0),
        tree: get_tree(ignore, path),
        path: String::from(path),
        file_stats,
        lang_stats,
        branches,
        log,
        status,
        file_time,
        tab: 0,
    };

    setup_terminal(&mut app)
}
