#[derive(Clone, Debug)]
pub enum Format {
    JSON,
    YAML,
}

#[derive(Debug)]
pub struct AppState {
    pub locale: String,
    pub path_config: String,
    pub path_in: String,
    pub path_out: String,
    pub format_in: Format,
    pub format_out: Format,
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub enable_opa: bool,
    pub opa_url: Option<String>,
}
