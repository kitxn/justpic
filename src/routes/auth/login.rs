use actix_web::{
    HttpResponse, post, web::{self, Json}
};

use crate::{
    database::repositories,
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
    state: web::Data<crate::state::State>,
    payload: Json<LoginRequestData>,
) -> Result<HttpResponse> {
    payload.validate()?;

    let user = repositories::users::fetch_by_username(&payload.username, state.db())
        .await?
        .ok_or(Error::InvalidCredentials)?;

    
    // TODO: move password checking in other task
    let is_valid_password = crate::util::crypto::bcrypt_validate(&payload.password, &user.password)?;
    if !is_valid_password {
      return Err(Error::InvalidCredentials);
    }

    // TODO: add session creating

    let res = UserResponse::from(user);
    Ok(HttpResponse::Ok().json(res))
}
