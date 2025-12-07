use actix_web::{
    HttpRequest, HttpResponse, patch,
    web::{self, Json},
};

use crate::{
    auth::sessions::extract_session_from_cookie,
    database::repositories,
    error::{Error, Result},
    models::users::change_password::ChangePasswordRequestData,
    util,
};

#[utoipa::path(
    patch, 
    path = "/api/users/me/password", 
    tag = "users", 
    request_body = ChangePasswordRequestData,
    responses(
        (
            status = 204,
            description = "Password changed"
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
#[patch("/me/password")]
pub async fn change_password(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Json<ChangePasswordRequestData>,
) -> Result<HttpResponse> {
    let session = extract_session_from_cookie(&req, state.db())
        .await?
        .ok_or(Error::Unauthorized)?;

    let hashed_password =
        tokio::task::spawn_blocking(move || util::crypto::bcrypt_hash(&payload.new_password))
            .await??;

    let query_res =
        repositories::users::change_password(&session.owner_id(), &hashed_password, state.db())
            .await?;

    if query_res.rows_affected() == 0 {
        // TODO: change error type
        return Err(Error::ItemNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}
