use actix_web::{
    HttpRequest, HttpResponse, patch,
    web::{self, Json},
};

use crate::{
    auth::sessions::extract_session_from_cookie,
    database::repositories,
    error::{Error, Result},
    models::users::change_username::ChangeUsernameRequestData,
    traits::validation::Validatable,
};

#[utoipa::path(
    patch, 
    path = "/api/users/me/username", 
    tag = "users", 
    request_body = ChangeUsernameRequestData,
    responses(
        (
            status = 204,
            description = "Username changed"
        ),
        (
            status = 401, 
            description = "Client is not authorized"
        ),
        (
            status = 404, 
            description = "User not found"
        ),
    )
)]
#[patch("/me/username")]
pub async fn change_username(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<ChangeUsernameRequestData>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let session = extract_session_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    let user_id = session.owner_id();

    let res = repositories::users::change_username(&user_id, &payload.username, state.db())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
            _ => Error::from(e),
        })?;

    if res.rows_affected() == 0 {
        return Err(Error::ItemNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}
