use std::path::Path;
use std::process::{Command, Stdio};

pub fn get_tree(ignore: bool, path: &str) -> String {
    if ignore && Path::new("./.gitignore").exists() {
        let git_ls = Command::new("git")
            .arg("ls-tree")
            .arg("-r")
            .arg("--name-only")
            .arg("HEAD")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let tree = if cfg!(target_os = "windows") {
            let tree = Command::new("cmd")
                .arg("/C")
                .arg("tree")
                .arg("--fromfile")
                .stdin(Stdio::from(git_ls.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap(); 
            tree   
        } else {
            let tree = Command::new("tree")
                .arg("--fromfile")
                .stdin(Stdio::from(git_ls.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            tree
        };

        return String::from_utf8(tree.wait_with_output().unwrap().stdout).unwrap();
    } else {
        if cfg!(target_os = "Windows") {
            let output1 = Command::new("cmd")
                .arg("/C")
                .arg("tree")
                .arg(path)
                .output()
                .expect("Tree command failed");
            return String::from_utf8(output1.stdout).unwrap();
        } else {
            let output1 = Command::new("tree")
                .arg(path)
                .output()
                .expect("Tree command failed");
            return String::from_utf8(output1.stdout).unwrap();
        }
    }
}

pub fn get_branches(path: &str) -> String {
    let branches = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("branch")
        .output()
        .expect("git branch command failed");

    let mut branches = String::from_utf8(branches.stdout).unwrap();
    if branches == "" {
        branches = String::from("Not a git repository. No branches found.");
    }
    branches
}

pub fn get_log_tree(path: &str) -> String {
    let log = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("-n 20")
        .arg("--graph")
        .arg("--pretty=oneline")
        .arg("--abbrev-commit")
        .output()
        .expect("git log command failed");
    let mut log = String::from_utf8(log.stdout).unwrap();
    if log == "" {
        log = String::from("Not a git repository. No log found.");
    }
    log
}

pub fn get_log(path: &str) -> String {
    let log = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("-n 5")
        .output()
        .expect("git log command failed");
    let mut log = String::from_utf8(log.stdout).unwrap();
    if log == "" {
        log = String::from("Not a git repository. No log found.");
    }
    log
}

pub fn get_status(path: &str) -> String {
    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .output()
        .expect("git status command failed");
    let mut status = String::from_utf8(status.stdout).unwrap();

    if status == "" {
        status = String::from("Not a git repository. No status found.");
    }
    status
}
