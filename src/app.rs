
#[derive(Debug, Clone)]
pub enum Format {
    JSON,
    YAML,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub locale: String,
    pub path_config: String,
    pub path_in: String,
    pub path_out: String,
    pub format_in: Format,
    pub format_out: Format,
    pub storage_in: Storage,
    pub storage_out: Storage,
}

#[derive(Debug, Clone)]
pub enum Storage {
    Filesystem(String),
    Sqlite(String),
}

impl Storage {
    pub fn from_str(s:&str) -> Self {
        debug!("Storage::from_str : Connection string is '{}'", s);
        if s.starts_with("sqlite:") {
            Storage::Sqlite(Self::get_path(s, 6))
        } else {
            Storage::Filesystem(Self::get_path(s, 0))
        }
    }

    fn get_path(s:&str, len:usize) -> String {
        let res = s.chars().skip(len).take(s.chars().count() - len).collect();
        debug!("Storage::get_path : Path string is '{}'", res);
        res
    }
}
