use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Json},
};

use crate::{
    SESSION_COOKIE_NAME,
    error::{Error, Result},
    models::{
        sessions::Session,
        users::requests::{
            UserChangePasswordRequest, UserChangeUsernameRequest, UserDeleteAccountRequest,
        },
    },
    repositories,
    traits::validation::Validatable,
    util,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(users_get))
            .service(
                web::scope("/me")
                    .route("", web::get().to(user_get_me))
                    .route("", web::delete().to(user_delete_me))
                    .route("/username", web::patch().to(user_update_me_username))
                    .route("/password", web::patch().to(user_update_me_password)),
            )
            .route("/by-name/{username}", web::get().to(user_get_by_username)),
    );
}

pub async fn user_get_me(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    // TODO: remove 2N db request
    let Some(mut session) = Session::from_request(&req, state.db()).await? else {
        return Err(Error::Unauthorized);
    };
    session.throw_error_if_expired()?;

    let Some(user) = repositories::users::get_by_session_id(session.id(), state.db()).await? else {
        return Err(Error::AccessDenied);
    };

    // Extend the session if its lifetime is approaching the end
    if (0..=2).contains(&session.days_of_life_left()) {
        session.extend_life_time(7);
        repositories::sessions::update_expire_datetime(session.id(), session.expires(), state.db())
            .await?;

        tracing::info!("Session lifetime extended: {}", session.id());

        return Ok(HttpResponse::Ok()
            .cookie(session.as_cookie())
            .json(user.to_public_model()));
    }

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

pub async fn users_get(state: web::Data<crate::state::State>) -> Result<HttpResponse> {
    let list = repositories::users::get_many(0, state.db()).await?;

    let res = list
        .into_iter()
        .map(|v| v.to_public_model())
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}

pub async fn user_get_by_username(
    state: web::Data<crate::state::State>,
    username_param: web::Path<String>,
) -> Result<HttpResponse> {
    let user = repositories::users::get_by_username(&username_param, state.db())
        .await?
        .ok_or(Error::ItemNotFound)?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

pub async fn user_update_me_username(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserChangeUsernameRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let session = Session::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;
    session.throw_error_if_expired()?;

    let username = payload.username.to_lowercase();

    let res = repositories::users::update_username(session.owner_id(), &username, state.db())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
            _ => Error::from(e),
        })?;

    if res.rows_affected() == 0 {
        return Err(Error::AccessDenied);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn user_update_me_password(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserChangePasswordRequest>,
) -> Result<HttpResponse> {
    let payload = payload.into_inner();
    payload.validate()?;

    let session = Session::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;
    session.throw_error_if_expired()?;

    let Some(user) = repositories::users::get_by_session_id(session.id(), state.db()).await? else {
        return Err(Error::AccessDenied);
    };

    let input_old_pw = payload.old_password;
    let hashed_pass = user.password().to_string();

    let is_valid_password = util::crypto::bcrypt_validate(&input_old_pw, &hashed_pass)?;
    if !is_valid_password {
        return Err(Error::InvalidCredentials);
    }

    let input_new_pw = payload.new_password;

    let hashed_password = util::crypto::bcrypt_hash(&input_new_pw)?;

    let query_res =
        repositories::users::update_password(session.owner_id(), &hashed_password, state.db())
            .await?;

    if query_res.rows_affected() == 0 {
        return Err(Error::AccessDenied);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn user_delete_me(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<UserDeleteAccountRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let session = Session::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;
    session.throw_error_if_expired()?;

    let Some(user) = repositories::users::get_by_session_id(session.id(), state.db()).await? else {
        return Err(Error::AccessDenied);
    };

    let input_pw = payload.password.clone();
    let hashed_pw = user.password().to_string();

    if !util::crypto::bcrypt_validate(&input_pw, &hashed_pw)? {
        return Err(Error::InvalidCredentials);
    }

    let id = user.id();

    let query_res = repositories::users::delete_by_id(id, state.db()).await?;
    if query_res.rows_affected() == 0 {
        tracing::warn!("Failed to delete user: {}", id);
        return Err(Error::AccessDenied);
    }

    tracing::info!("The user was deleted: {}-{}", user.username(), id);

    let cookie = util::cookie::remove(SESSION_COOKIE_NAME);
    Ok(HttpResponse::NoContent().cookie(cookie).finish())
}
