extern crate walkdir;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self};
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
use walkdir::WalkDir;

pub fn count_lines(file: &Path) -> u32 {
    let contents = fs::read_to_string(file);
    if let Ok(lines) = contents {
        lines.lines().count().try_into().unwrap()
    } else {
        0
    }
}

pub fn get_stats(
    path: &str,
    ignore: &mut bool,
) -> (HashMap<String, (u32, u32, u64)>, u64, HashMap<String, u64>) {
    let mut file_counts: HashMap<String, (u32, u32, u64)> = HashMap::new();
    let mut times: HashMap<String, u64> = HashMap::new();
    let mut proj_size: u64 = 0;

    let ext_map = HashMap::from([
        ("rs", String::from(" Rust")),
        ("r", String::from("   R")),
        ("c", String::from("  C")),
        ("cs", String::from("  C#")),
        ("cpp", String::from("  C++")),
        ("java", String::from(" Java")),
        ("scala", String::from("Scala")),
        ("erl", String::from("Erlang")),
        ("html", String::from(" HTML")),
        ("css", String::from(" CSS")),
        ("ejs", String::from(" EJS")),
        ("rb", String::from(" Ruby")),
        ("php", String::from(" PHP")),
        ("js", String::from("JavaScript")),
        ("ts", String::from("TypeScript")),
        ("py", String::from("Python")),
        ("go", String::from("  Go")),
        ("bash", String::from(" Bash")),
    ]);

    let ls = Command::new("git")
        .arg("ls-files")
        .output()
        .expect("Git ls-files failed");
    let ls = String::from_utf8(ls.stdout).expect("Can't convert git ls-files to string");
    let git_ls: Vec<&str> = ls.split("\n").collect();

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            if *ignore {
                let path2 = file
                    .path()
                    .strip_prefix("./")
                    .expect("Coudn't strip prefix")
                    .to_str()
                    .unwrap();
                if !git_ls.contains(&path2) {
                    continue;
                }
            }

            let t = Path::new(file.file_name())
                .extension()
                .and_then(OsStr::to_str);

            let ext = match t {
                Some(ext) => ext,
                None => "Undefined",
            };

            let extensions: Vec<&str> = ext_map.keys().cloned().collect();
            if extensions.iter().any(|e| ext.eq(*e)) {
                let mtime = file.metadata().unwrap().modified().expect("0");
                let mtime = SystemTime::now()
                    .duration_since(mtime)
                    .expect("File time error")
                    .as_secs();

                times.insert(file.file_name().to_str().expect("msg").to_string(), mtime);
                let size = file.metadata().unwrap().len();
                let lc = count_lines(file.path());
                proj_size += size;

                if let Some(extension) = ext_map.get(&ext) {
                    if let Some(count) = file_counts.get(extension) {
                        file_counts.insert(
                            extension.to_string(),
                            (count.0 + 1, count.1 + lc, count.2 + size),
                        );
                    } else {
                        file_counts.insert(extension.to_string(), (1, lc, size));
                    }
                }
            }
        }
    }
    (file_counts, proj_size, times)
}

pub fn get_percentages(
    stats: &HashMap<String, (u32, u32, u64)>,
    proj_size: u64,
) -> HashMap<String, f64> {
    let mut lang: HashMap<String, f64> = HashMap::new();

    for file in stats {
        let (_, _, b) = file.1;
        let ps = *b as f64 / proj_size as f64;
        lang.insert(file.0.to_string(), ps * 100.0);
    }

    lang
}
