use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Json},
};

use crate::{
    SESSION_COOKIE_NAME,
    error::{Error, Result},
    models::{
        sessions::{Session, requests::create::SessionCreateRequest},
        users::{User, requests::UserCreateRequest},
    },
    repositories,
    traits::validation::Validatable,
    util,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/register", web::post().to(register)),
    );
}

pub async fn login(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<SessionCreateRequest>,
) -> Result<HttpResponse> {
    let session = Session::from_request(&req, state.db()).await?;
    if session.is_some_and(|v| !v.is_expired()) {
        return Err(Error::Conflict);
    }

    payload.validate()?;

    let user = repositories::users::get_by_username(&payload.username, state.db())
        .await?
        .ok_or(Error::InvalidCredentials)?;

    // TODO: move password checking in other task
    let is_valid_password =
        crate::util::crypto::bcrypt_validate(&payload.password, user.password())?;
    if !is_valid_password {
        return Err(Error::InvalidCredentials);
    }

    // TODO: add user agent getting
    let session = Session::new(user.id_copy());
    repositories::sessions::insert(&session, state.db()).await?;

    let cookie = session.as_cookie();
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(user.to_public_model()))
}

pub async fn logout(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let session = Session::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    repositories::sessions::remove_by_id(session.id(), state.db()).await?;

    let cookie = util::cookie::remove(SESSION_COOKIE_NAME);
    Ok(HttpResponse::NoContent().cookie(cookie).finish())
}

pub async fn register(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserCreateRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let session = Session::from_request(&req, state.db()).await?;
    if session.is_some_and(|v| !v.is_expired()) {
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

    let session = Session::new(user.id_copy());
    repositories::sessions::insert(&session, state.db()).await?;

    let cookie = session.as_cookie();

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(user.to_public_model()))
}
