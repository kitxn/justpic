use actix_web::{
    HttpRequest, HttpResponse, post,
    web::{self, Json},
};

use crate::{
    auth::sessions::{create_session_cookie, extract_session_from_cookie},
    database::{
        repositories,
        schemas::{sessions::DbSession, users::DbUser},
    },
    error::{Error, Result},
    models::auth::register::{RegisterRequestData, RegisterResponseData},
    traits::validation::Validatable,
    util,
};


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
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<RegisterRequestData>,
) -> Result<HttpResponse> {
    if extract_session_from_cookie(&req, state.db())
        .await?
        .is_some()
    {
        return Err(Error::Conflict);
    }

    payload.validate()?;

    let payload = payload.into_inner();

    let username = payload.username;

    let password = payload.password;
    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&password)).await??;

    let user = DbUser::new(username.clone(), hashed_password);
    repositories::users::insert(&user, state.db())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
            _ => Error::from(e),
        })?;

    let session = DbSession::new(user.id, None);
    repositories::sessions::insert(&session, state.db()).await?;

    let cookie = create_session_cookie(&session);

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(RegisterResponseData {
            message: "Registered",
            username,
        }))
}
