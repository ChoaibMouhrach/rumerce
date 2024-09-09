use serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreRoleSchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateRoleSchema {
    pub name: String,
}
