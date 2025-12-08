use actix_web::{
    HttpRequest, HttpResponse, post,
    web::{self, Json},
};

use crate::{
    auth::sessions::{create_session_cookie, extract_session_from_cookie},
    database::{repositories, schemas::sessions::DbSession},
    error::{Error, Result},
    models::{auth::login::UserLoginRequest, users::UserPublicModel},
    traits::validation::Validatable,
};

#[utoipa::path(
    post,
    description = "Log in to account",
    path = "/auth/login", 
    tag = "auth", 
    request_body = UserLoginRequest,
    responses(
        (
            status = 200, 
            body = UserPublicModel,
            description = "Successful login"
        ),
        (
            status = 401, 
            description = "Invalid login credentials"
        ),
        (
            status = 409, 
            description = "Already logged in"
        ),
    )
)]
#[post("/login")]
pub async fn login(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserLoginRequest>,
) -> Result<HttpResponse> {
    if extract_session_from_cookie(&req, state.db())
        .await?
        .is_some()
    {
        return Err(Error::Conflict);
    }

    payload.validate()?;

    let user = repositories::users::fetch_by_username(&payload.username, state.db())
        .await?
        .ok_or(Error::InvalidCredentials)?;

    // TODO: move password checking in other task
    let is_valid_password =
        crate::util::crypto::bcrypt_validate(&payload.password, &user.password)?;
    if !is_valid_password {
        return Err(Error::InvalidCredentials);
    }

    // TODO: add user agent getting
    let session = DbSession::new(user.id, None);
    repositories::sessions::insert(&session, state.db()).await?;

    let cookie = create_session_cookie(&session);
    let res = UserPublicModel::from(user);
    Ok(HttpResponse::Ok().cookie(cookie).json(res))
}
