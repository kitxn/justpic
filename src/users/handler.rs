use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Data, Json},
};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    sessions,
    state::State,
    types::pagination::PaginationParams,
    users::{
        self,
        models::{
            UserChangePasswordRequest, UserChangeUsernameRequest, UserCreateRequest,
            UserDeleteRequest,
        },
    },
    utils::validation::Validatable,
};

/// Shared domain for working with users
const DOMAIN_NAME: &str = "/users";
/// Subdomain for working with the current user
const SELF_DOMAIN_NAME: &str = "/me";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(DOMAIN_NAME)
            .service(
                web::scope(SELF_DOMAIN_NAME)
                    .route("", web::get().to(get_me))
                    .route("/username", web::patch().to(update_me_username))
                    .route("/password", web::patch().to(update_me_password))
                    .route("", web::delete().to(delete_me)),
            )
            .route("", web::get().to(get_all))
            .route("/id/{id}", web::get().to(get_by_id))
            .route("/u/{username}", web::get().to(get_by_username)),
    );

    cfg.route("/auth/register", web::post().to(create_new));
}

async fn create_new(
    state: Data<State>,
    req: HttpRequest,
    payload: Json<UserCreateRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let existed_session = sessions::http::try_extract_from_req(&req, state.db()).await?;
    sessions::guards::except_active_session(existed_session)?;

    let user =
        super::domain::create_and_insert(&payload.username, &payload.password, state.db()).await?;

    let new_session = sessions::create_and_insert(user.id_copy(), state.db()).await?;

    Ok(HttpResponse::Created()
        .cookie(sessions::http::generate_cookie(&new_session))
        .json(user.to_public_model()))
}

async fn get_me(state: Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let client_session = sessions::http::extract_from_req(&req, state.db()).await?;
    sessions::guards::except_expired_session(&client_session)?;

    let user = users::queries::try_get_by_id(client_session.owner_id(), state.db())
        .await?
        .ok_or(Error::AccessDenied)?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

// TODO: add query-pagination
async fn get_all(state: Data<State>) -> Result<HttpResponse> {
    let pagination = PaginationParams::new(0, 50);
    let users = users::get_all(pagination, state.db()).await?;

    let res = users
        .into_iter()
        .map(|v| v.to_public_model())
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}

async fn get_by_id(state: Data<State>, id: web::Path<Uuid>) -> Result<HttpResponse> {
    let user = users::queries::get_by_id(&id, state.db()).await?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

async fn get_by_username(state: Data<State>, username: web::Path<String>) -> Result<HttpResponse> {
    let user = users::queries::get_by_username(&*username, state.db()).await?;

    Ok(HttpResponse::Ok().json(user.to_public_model()))
}

async fn update_me_username(
    state: Data<State>,
    req: HttpRequest,
    payload: Json<UserChangeUsernameRequest>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let session = sessions::http::extract_from_req(&req, state.db()).await?;
    sessions::guards::except_expired_session(&session)?;

    Ok(HttpResponse::NotImplemented().finish())
}

async fn update_me_password(
    state: Data<State>,
    req: HttpRequest,
    payload: Json<UserChangePasswordRequest>,
) -> Result<HttpResponse> {
    let session = sessions::http::extract_from_req(&req, state.db()).await?;
    sessions::guards::except_expired_session(&session)?;

    Ok(HttpResponse::NotImplemented().finish())
}

async fn delete_me(
    state: Data<State>,
    req: HttpRequest,
    payload: Json<UserDeleteRequest>,
) -> Result<HttpResponse> {
    let session = sessions::http::extract_from_req(&req, state.db()).await?;
    sessions::guards::except_expired_session(&session)?;

    Ok(HttpResponse::NotImplemented().finish())
}
