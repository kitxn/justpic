use actix_web::{
    HttpResponse, post,
    web::{self, Json},
};

use crate::{
    database::{repositories, schemas::users::User},
    error::{Error, Result},
    models::auth::register::{RegisterRequestData, RegisterResponseData},
    traits::validation::Validatable,
    util,
};

/// Register new user
#[utoipa::path(
    post, 
    path = "/api/auth/register", 
    tag = "auth", 
    request_body = RegisterRequestData,
    responses(
        (
            status = 201, 
            body = RegisterResponseData,
            description = "Successful registration"),
        (
            status = 400, 
            description = "Invalid data for registration"
        ),
    )
)]
#[post("/register")]
pub async fn register(
    state: web::Data<crate::state::State>,
    payload: Json<RegisterRequestData>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let payload = payload.into_inner();

    let username = payload.username;

    let password = payload.password;
    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&password)).await??;

    let user = User::new(username.clone(), hashed_password);
    repositories::users::insert(&user, state.db())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
            _ => Error::from(e),
        })?;

    Ok(HttpResponse::Created().json(RegisterResponseData {
        message: "Registered",
        username,
    }))
}
