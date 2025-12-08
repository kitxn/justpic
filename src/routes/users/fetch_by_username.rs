use actix_web::{HttpResponse, get, web};

use crate::{
    database::repositories,
    error::{Error, Result},
    models::users::UserPublicModel,
};

#[utoipa::path(
    get, 
    description = "Get information about a user by their username",
    path = "/users/by-name/{username}", 
    tag = "users", 
    responses(
        (
            status = 200,
            body = UserPublicModel,
            description = "Information about the user with given username",
        ),
        (
            status = 404, 
            description = "User not found"
        ),
    )
)]
#[get("/by-name/{username}")]
pub async fn fetch_by_username(
    state: web::Data<crate::state::State>,
    username: web::Path<String>,
) -> Result<HttpResponse> {
    let user = repositories::users::fetch_by_username(&username, state.db())
        .await?
        .ok_or(Error::ItemNotFound)?;

    let res = UserPublicModel::from(user);
    Ok(HttpResponse::Ok().json(res))
}
