use serde::Deserialize;
use validator::Validate;

pub struct StoreSettingsSchema {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Validate)]
pub struct SetupSchema {
    #[validate(email)]
    pub email: String,
}
