extern crate walkdir;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn count_lines(file: &Path) -> u32 {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    contents.lines().count().try_into().unwrap()
}

pub fn list_files(path: &str) -> (HashMap<String, (u32, u32, u64)>, u64) {
    let mut file_counts: HashMap<String, (u32, u32, u64)> = HashMap::new();
    let ext_map = HashMap::from([
        ("rs", String::from("Rust")),
        ("json", String::from("JSON")),
        ("py", String::from("Python")),
    ]);
    let mut proj_size: u64 = 0;

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let t = Path::new(file.file_name())
                .extension()
                .and_then(OsStr::to_str);

            let ext = match t {
                Some(ext) => ext,
                None => "Undefined",
            };

            let extensions = vec!["rs", "json", "py"];
            if extensions.iter().any(|e| ext.contains(e)) {
                let size = file.metadata().unwrap().len();
                proj_size += size;
                let lc = count_lines(file.path());
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
    // println!("{:?}", file_counts);
    (file_counts, proj_size)
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
