use actix_multipart::Multipart;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, get, post},
};

use crate::{
    error::{Error, Result},
    models::{cards::builder::MultipartCardBuilder, files::state::FileState, sessions::Session},
    repositories,
    util::multipart::parse_multipart,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cards")
            .route("", get().to(get_all))
            .route("/", post().to(create_new)),
    );
}

/// ### Upload a new card to the storage
/// The endpoint is responsible for loading and
/// creating new cards from a multipart form.
///
/// ### Structure of models
/// `Multipart` -> `Vec<MultipartSegment>` ->
/// `MultipartCardBuilder` -> `Card` -> `CardApiModel`
pub async fn create_new(
    req: HttpRequest,
    state: web::Data<crate::state::State>,
    payload: Multipart,
) -> Result<HttpResponse> {
    // Extracting a session from a cookie
    let Some(session) = Session::from_request(&req, state.db()).await? else {
        return Err(Error::Unauthorized);
    };
    session.throw_error_if_expired()?;

    // Multipart parsing
    let fields = parse_multipart(&state, payload).await?;

    // We create a card and file model from the multipart fields
    let mut card_builder = MultipartCardBuilder::new();
    for seg in fields {
        card_builder.handle_segment(seg);
    }

    let card = card_builder.build(*session.owner_id())?;

    // Inserting cards and files into the database
    let mut tx = state.db().begin().await?;
    {
        repositories::files::insert(card.file(), &mut *tx).await?;
        repositories::cards::insert(&card, &mut *tx).await?;
    }
    tx.commit().await?;

    // Moving a file from temporary storage to permanent storage
    let move_res = state
        .temp_store()
        .move_to_another(state.store(), card.file().id())
        .await;

    if let Err(e) = move_res {
        tracing::error!("Failed to move file: {e}");
        repositories::files::update_file_state(card.file().id(), FileState::Failed, state.db())
            .await?;
    }

    let res = card.clone().to_api_model();

    // Run background media processing in a separate thread
    tokio::spawn(async move {
        // Transferring ownership to the processing thread
        let _card = card;
        let _state = state;

        // TODO!
    });

    // Converting a card into a public API model and return it
    Ok(HttpResponse::Created().json(res))
}

pub async fn get_all(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_by_owner(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_me(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_by_id(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_title(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_description(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_by_id(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}
