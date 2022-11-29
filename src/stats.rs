extern crate walkdir;
use gitignore;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;

pub fn count_lines(file: &Path) -> u32 {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    contents.lines().count().try_into().unwrap()
}

pub fn list_files(
    path: &str,
    ignore: &mut bool,
) -> (HashMap<String, (u32, u32, u64)>, u64, HashMap<String, u64>) {
    let mut file_counts: HashMap<String, (u32, u32, u64)> = HashMap::new();
    let mut times: HashMap<String, u64> = HashMap::new();
    let mut proj_size: u64 = 0;
    let ext_map = HashMap::from([
        ("rs", String::from("Rust")),
        ("r", String::from("R")),
        ("c", String::from("C")),
        ("cs", String::from("C#")),
        ("cpp", String::from("C++")),
        ("java", String::from("Java")),
        ("scala", String::from("Scala")),
        ("erl", String::from("Erlang")),
        ("html", String::from("HTML")),
        ("css", String::from("CSS")),
        ("ejs", String::from("EJS")),
        ("rb", String::from("Ruby")),
        ("php", String::from("PHP")),
        ("js", String::from("JavaScript")),
        ("ts", String::from("TypeScript")),
        ("py", String::from("Python")),
        ("go", String::from("Go")),
        ("bash", String::from("Bash")),
    ]);

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let t = Path::new(file.file_name())
                .extension()
                .and_then(OsStr::to_str);

            let ext = match t {
                Some(ext) => ext,
                None => "Undefined",
            };

            let extensions: Vec<&str> = ext_map.keys().cloned().collect();
            if extensions.iter().any(|e| ext.eq(*e)) {
                if *ignore {
                    let gfile = gitignore::File::new(Path::new("./.gitignore"));
                    if let Ok(gfile1) = gfile {
                        let gfile = gfile1;
                        if gfile.is_excluded(file.path()).unwrap() {
                            continue;
                        }
                    } else {
                        *ignore = false;
                    }
                }

                let mtime = file.metadata().unwrap().modified().expect("0");
                let mtime = mtime
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("File thinks it was created before Epoch")
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

    // println!("{:?}", lang);
    lang
}
