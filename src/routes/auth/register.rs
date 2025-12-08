use actix_web::{
    HttpRequest, HttpResponse, post,
    web::{self, Json},
};

use crate::{
    auth::sessions::{create_session_cookie, extract_session_from_cookie},
    database::{repositories, schemas::sessions::DbSession},
    error::{Error, Result},
    models::
        users::{User, requests::UserCreateRequest, responses::common::UserPublic}
    ,
    traits::validation::Validatable,
    util,
};

#[utoipa::path(
    post, 
    description = "Register a new account",
    path = "/auth/register", 
    tag = "auth", 
    request_body = UserCreateRequest,
    responses(
        (
            status = 201, 
            body = UserPublic,
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
    payload: Json<UserCreateRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    if extract_session_from_cookie(&req, state.db())
        .await?
        .is_some()
    {
        return Err(Error::Conflict);
    }

    let payload = payload.into_inner();

    let password = payload.password;
    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&password)).await??;

    let user = User::new(&payload.username, hashed_password);

    repositories::users::insert(&user, state.db())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
            _ => Error::from(e),
        })?;

    let session = DbSession::new(*user.id(), None);
    repositories::sessions::insert(&session, state.db()).await?;

    let cookie = create_session_cookie(&session);

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(user.to_public_model()))
}
