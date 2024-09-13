use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreRoleSchema {
    #[validate(length(min = 1))]
    pub name: String,
}
