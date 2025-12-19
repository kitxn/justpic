use actix_multipart::Multipart;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, get, post},
};
use futures::{StreamExt, TryStreamExt};

use crate::{
    error::{Error, Result},
    models::{files::uploading::UploadedFile, sessions::Session},
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
    mut payload: Multipart,
) -> Result<HttpResponse> {
    let Some(mut session) = Session::from_request(&req, state.db()).await? else {
        return Err(Error::Unauthorized);
    };
    session.throw_error_if_expired()?;

    let cfg = state.cfg();

    // let mut entity = UploadingFileDataConstructor::new(*session.owner_id());

    // // Payload handling cycle
    // while let Some(field_res) = payload.next().await {
    //     let mut field = field_res.map_err(|_| Error::BadInput)?;

    //     // If the field has no name, skip the iteration.
    //     let Some(field_name) = field.name() else {
    //         continue;
    //     };

    //     match field_name {
    //         "file" => {
    //             // entity.with_stream(field);
    //         }
    //         "title" => {

    //             // entity.with_title(field.cont);
    //         }
    //         "description" => {}
    //         _ => {
    //             // If the name is unknown, skip it.
    //             continue;
    //         }
    //     }
    // }

    Ok(HttpResponse::Ok().finish())
}

pub async fn cards_get_all(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
