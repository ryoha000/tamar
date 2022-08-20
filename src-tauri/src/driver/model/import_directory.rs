use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirPathInfo {
    pub path: String,
    pub name: String,
    pub dir_deps: Vec<DirDeps>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirDeps {
    pub deps: u8,
    pub name: String,
}
