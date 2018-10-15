
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
    Filesystem,
    Sqlite,
}

impl Storage {
    pub fn from_str(s:&str) -> Self {
        debug!("Storage::from_str : Connection string is '{}'", s);
        if s.starts_with("sqlite:") {
            Storage::Sqlite
        } else {
            Storage::Filesystem
        }
    }
}
