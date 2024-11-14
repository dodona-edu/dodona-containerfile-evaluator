use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub user: Option<String>,
    pub workdir: Option<String>,
}
