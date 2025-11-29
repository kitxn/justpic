use actix_web::{HttpResponse, get, web};

use crate::{
    database::repositories,
    error::{Error, Result},
    models::users::UserResponse,
};

/// Fetch user by its username
#[utoipa::path(
    get, 
    path = "/api/users/{username}", 
    tag = "users", 
    responses(
        (
            status = 200,
        ),
        (
            status = 404, 
            description = "User not found"
        ),
    )
)]
#[get("/{username}")]
pub async fn fetch_by_username(
    state: web::Data<crate::state::State>,
    username: web::Path<String>,
) -> Result<HttpResponse> {
    let user = repositories::users::fetch_by_username(&username, state.db())
        .await?
        .ok_or(Error::ItemNotFound)?;

    let res = UserResponse::from(user);
    Ok(HttpResponse::Ok().json(res))
}
