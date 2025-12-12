use crate::models::users::{requests, responses::common};

#[utoipa::path(
    get, 
    path = "/users/me", 
    tag = "users.me", 
    responses(
        (
            status = 200,
            body = common::UserPublic,
        ),
        (
            status = 401, 
        ),
    )
)]
pub fn get_me() {}

#[utoipa::path(
    get, 
    path = "/users", 
    tag = "users", 
    responses(
        (
            status = 200,
            body = Vec<common::UserPublic>,
        ),
    )
)]
pub fn get_all() {}

#[utoipa::path(
    get, 
    path = "/users/by-name/{username}", 
    tag = "users", 
    responses(
        (
            status = 200,
            body = common::UserPublic,
        ),
        (
            status = 404, 
        ),
    )
)]
pub fn get_by_username() {}


#[utoipa::path(
    patch, 
    path = "/users/me/username", 
    tag = "users.me", 
    request_body = requests::UserChangeUsernameRequest,
    responses(
        (
            status = 204,
        ),
        (
            status = 401, 
        ),
    )
)]
pub fn update_me_username() {}

#[utoipa::path(
    patch,
    path = "/users/me/password", 
    tag = "users.me", 
    request_body = requests::UserChangePasswordRequest,
    responses(
        (
            status = 204,
        ),
        (
            status = 401, 
        ),
    )
)]
pub fn update_me_password() {}

#[utoipa::path(
    delete,
    path = "/users/me", 
    tag = "users.me", 
    request_body = requests::UserDeleteAccountRequest,
    responses(
        (
            status = 204,
        ),
        (
            status = 401, 
        ),
        (
            status = 403,
        )
    )
)]
pub fn delete_me() {}
