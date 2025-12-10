use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Json},
};

use crate::{
    error::{Error, Result},
    models::{
        sessions::Session,
        users::{
            User,
            requests::{UserChangePasswordRequest, UserChangeUsernameRequest},
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
                    .route("/me/username", web::patch().to(user_update_me_username))
                    .route("/me/password", web::patch().to(user_update_me_password)),
            )
            .route("/by-name/{username}", web::get().to(user_get_by_username)),
    );
}

pub async fn user_get_me(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
) -> Result<HttpResponse> {
    let user = User::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

pub async fn users_get() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("TODO ENDPOINT!"))
}

pub async fn user_get_by_username(
    state: web::Data<crate::state::State>,
    username_param: web::Path<String>,
) -> Result<HttpResponse> {
    let user = repositories::users::fetch_by_username(&username_param, state.db())
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
    payload.validate()?;

    // TODO: add old password checking

    let session = Session::from_request(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&payload.new_password))
            .await??;

    let query_res =
        repositories::users::update_password(session.owner_id(), &hashed_password, state.db())
            .await?;

    if query_res.rows_affected() == 0 {
        return Err(Error::AccessDenied);
    }

    Ok(HttpResponse::NoContent().finish())
}
