use crate::file::File;
use std::fs;

pub fn find(path: &str, excludes: &Vec<String>) -> Result<Vec<File>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(path)?;
    let mut files: Vec<File> = Vec::new();

    for ref e in entries {
        let entry = e.as_ref().unwrap();
        let entry_pathbuf = entry.path();
        let entry_path = entry_pathbuf.to_str().unwrap();

        if excludes.into_iter().any(|f| entry_path.contains(f)) {
            // println!("skipping {}", entry_path);
            continue;
        }

        if entry.metadata().unwrap().is_dir() {
            if let Ok(fls) = &mut find(entry_path, excludes) {
                files.append(fls);
            }
            continue;
        }

        let byte_data = fs::read(entry_path).unwrap();
        match std::str::from_utf8(&byte_data) {
            Ok(str_data) => {
                let mut found_todos = find_todos(entry_path, str_data);
                files.append(&mut found_todos);
            }
            _ => {}
        }
    }

    Ok(files)
}

fn find_todos(path: &str, txt: &str) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    let lines = txt.split("\n");

    for (i, line) in lines.enumerate() {
        if let Some(l) = line.find("TODO:") {
            const SIZE_TODO_STR: usize = 5;
            let line_str: String = line.chars().skip(l + SIZE_TODO_STR).collect();

            files.push(File {
                data: line_str.trim_start().to_string(),
                line: (i + 1) as u32,
                path: path.to_string(),
            });
        }
    }

    files
}
