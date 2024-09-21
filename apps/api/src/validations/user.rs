use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StoreUserSchema<'a> {
    #[validate(length(min = 1))]
    pub name: Option<&'a str>,
    #[validate(email)]
    pub email: &'a str,
    pub role_id: Uuid,
}
