use actix_web::{
    HttpRequest, HttpResponse, post,
    web::{self, Json},
};

use crate::{
    auth::sessions::{create_session_cookie, extract_session_from_cookie},
    database::{repositories, schemas::sessions::DbSession},
    error::{Error, Result},
    models::{auth::login::LoginRequestData, users::UserResponse},
    traits::validation::Validatable,
};

/// Login
#[utoipa::path(
    post, 
    path = "/api/auth/login", 
    tag = "auth", 
    request_body = LoginRequestData,
    responses(
        (
            status = 200, 
            body = UserResponse,
            description = "Successful login"),
        (
            status = 401, 
            description = "Invalid login credentials"
        ),
    )
)]
#[post("/login")]
pub async fn login(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<LoginRequestData>,
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
    let res = UserResponse::from(user);
    Ok(HttpResponse::Ok().cookie(cookie).json(res))
}
