use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Data, Json, Path},
};

use crate::{
    SESSION_COOKIE_NAME,
    error::{Error, Result},
    sessions::{self, models::SessionCreateRequest},
    state::State,
    users,
    util::{self, crypto, validation::Validatable},
};

const DOMAIN_NAME: &str = "/sessions";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(DOMAIN_NAME)
            .route("", web::post().to(create_new))
            .route("", web::get().to(get_all))
            .route("/", web::delete().to(delete_all))
            .route("/current", web::delete().to(delete_current))
            .route("/{id}", web::delete().to(delete_by_id)),
    );
}

/// Login to account and create a new session
async fn create_new(
    state: Data<State>,
    payload: Json<SessionCreateRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let user = users::queries::try_get_by_username(&payload.username, state.db())
        .await?
        .ok_or(Error::InvalidCredentials)?;

    if !crypto::verify_password(&payload.password, user.password())? {
        return Err(Error::InvalidCredentials);
    }

    let new_session = sessions::create_and_insert(user.id_copy(), state.db()).await?;

    Ok(HttpResponse::Created()
        .cookie(sessions::http::generate_cookie(&new_session))
        .json(user.to_public_model()))
}

/// Get all sessions for the currently logged in account
async fn get_all(state: Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let session = sessions::http::extract_from_req(&req, state.db()).await?;
    todo!()
}

/// Logout and delete the session
async fn delete_current(state: Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let session = sessions::http::extract_from_req(&req, state.db()).await?;

    sessions::delete_by_id(*session.id(), state.db()).await?;

    Ok(HttpResponse::NoContent()
        .cookie(sessions::http::generate_empty_auth_cookie())
        .finish())
}

/// Remotely delete a session by ID
async fn delete_by_id(
    state: Data<State>,
    req: HttpRequest,
    id: Path<uuid::Uuid>,
) -> Result<HttpResponse> {
    let current_session = sessions::http::extract_from_req(&req, state.db()).await?;

    let target_session = sessions::queries::get_by_id(*id, state.db()).await?;

    if current_session.owner_id() != target_session.owner_id() {
        return Err(Error::AccessDenied);
    }

    sessions::delete_by_id(*target_session.id(), state.db()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete all user sessions
async fn delete_all(state: Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let session = sessions::http::extract_from_req(&req, state.db()).await?;
    todo!()
}
