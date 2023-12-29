use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Lab {
    pub name: String,
    pub path: String
}

impl Clone for Lab {
    fn clone(&self) -> Self {
        Lab {
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}