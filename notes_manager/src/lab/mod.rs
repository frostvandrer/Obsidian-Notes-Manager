use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Lab {
    pub name: String,
    pub path: String
}