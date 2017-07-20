#[derive(Clone)]
pub enum Format {
    JSON,
    YAML,
}

pub struct AppState {
    pub locale: String,
    pub path_config: String,
    pub path_in: String,
    pub path_out: String,
    pub format_in: Format,
    pub format_out: Format,
}
