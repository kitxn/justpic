use actix_multipart::Multipart;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, get, post},
};

use crate::{
    error::{Error, Result},
    models::sessions::Session,
    util::multipart::parse_multipart,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cards")
            .route("", get().to(cards_get_all))
            .route("/", post().to(card_create_new)),
    );
}

pub async fn card_create_new(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Multipart,
) -> Result<HttpResponse> {
    let Some(session) = Session::from_request(&req, state.db()).await? else {
        return Err(Error::Unauthorized);
    };
    session.throw_error_if_expired()?;

    let fields = parse_multipart(&state, payload).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn cards_get_all(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
