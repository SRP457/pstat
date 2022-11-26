extern crate walkdir;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn count_lines(file: &Path) -> u32 {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    contents.lines().count().try_into().unwrap()
}

fn list_files(path: &str) {
    let mut file_counts: HashMap<&str, (u32, u32, u64)> = HashMap::new();
    let ext_map = HashMap::from([
        ("toml", "TOML"),
        ("rs", "Rust"),
        ("json", "JSON"),
        ("txt", "Txt"),
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

            let extensions = vec!["rs", "toml", "json", "txt"];
            if extensions.iter().any(|e| ext.contains(e)) {
                let size = file.metadata().unwrap().len();
                let lc = count_lines(file.path());
                if let Some(extension) = ext_map.get(&ext) {
                    if let Some(count) = file_counts.get(extension) {
                        file_counts.insert(extension, (count.0 + 1, count.1 + lc, count.2 + size));
                    } else {
                        file_counts.insert(extension, (1, lc, size));
                    }
                }
            }
        }
    }

    println!("{:?}", file_counts);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => path,
        None => "None",
    };
    list_files(path);
}
