use colored::Colorize;
use std::fmt;

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub line: u32,
    pub data: String,
}

impl From<&File> for String {
    fn from(f: &File) -> Self {
        let mut path_str = format!("{}:{}", f.path, f.line).green().bold().to_string();
        path_str.push_str(&format!(" -> {}", f.data));
        path_str
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", String::from(self))
    }
}
