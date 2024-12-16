use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub from: Option<From>,
    pub user: Option<String>,
    pub workdir: Option<String>,
    pub expose: Option<Vec<Port>>,
    pub comments: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct From {
    pub image: String,
    pub tag: Option<String>,
    pub hash: Option<String>,
}

#[derive(Deserialize)]
pub struct Port {
    pub number: u16,
    pub protocol: Option<String>
}
