use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StoreUnitSchema<'a> {
    #[validate(length(min = 1))]
    pub name: &'a str,
}
