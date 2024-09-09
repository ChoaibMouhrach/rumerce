use serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreUnitSchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateUnitSchema {
    pub name: String,
}
