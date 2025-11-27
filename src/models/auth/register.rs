use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequestData {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "hunter42!")]
    pub password: String,

    #[schema(example = "hunter42!")]
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseData {
    #[schema(example = "Successful registration")]
    pub message: String,

    #[schema(example = "john_doe")]
    pub username: String,
}
