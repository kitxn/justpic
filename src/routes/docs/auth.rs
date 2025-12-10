use crate::models::{sessions::requests::create::SessionCreateRequest, users::{requests::UserCreateRequest, responses::common::UserPublic}};

#[utoipa::path(
    post,
    path = "/auth/login", 
    tag = "auth", 
    request_body = SessionCreateRequest,
    responses(
        (
            status = 200, 
            body = UserPublic,
        ),
        (
            status = 401, 
        ),
        (
            status = 409, 
        ),
    )
)]
pub fn login() {}

#[utoipa::path(
    post, 
    path = "/auth/logout", 
    tag = "auth", 
    responses(
        (
            status = 204, 
        ),
        (
            status = 401, 
        ),
    )
)]
pub fn logout() {}

#[utoipa::path(
    post, 
    path = "/auth/register", 
    tag = "auth", 
    request_body = UserCreateRequest,
    responses(
        (
            status = 201, 
            body = UserPublic,
        ),
        (
            status = 400, 
        ),
    )
)]
pub fn register() {}
