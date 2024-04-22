use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUrlSchema {
    pub url: String,
    pub code: Option<String>,
}
