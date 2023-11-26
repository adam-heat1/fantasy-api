use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateNewsBlurb {
    #[validate(length(min = 1, max = 255))]
    pub source: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 255))]
    pub description: String,
    #[validate(length(min = 1, max = 255))]
    pub link: String,
    #[validate(length(min = 1, max = 255))]
    pub date: String,
}
