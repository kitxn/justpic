use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::{
    database::{DatabaseError, DatabasePool},
    error::Error,
};

#[derive(Debug, FromRow)]
pub struct Card {
    pub(super) id: i64,
    pub(super) file_key: String,

    pub(super) mimetype: String,
    pub(super) filesize: i64,

    pub(super) width: u32,
    pub(super) height: u32,

    pub(super) title: Option<String>,
    pub(super) description: Option<String>,

    pub(super) created: DateTime<Utc>,

    pub(super) source_url: Option<String>,

    pub(super) is_private: bool,
    pub(super) state: CardState,
}

impl Card {
    /// Create a new [`Card`] entity
    pub fn new(
        title: Option<String>,
        description: Option<String>,
        source_url: Option<String>,
        mimetype: String,
        filesize: i64,
        is_private: bool,
    ) -> Self {
        Card {
            id: crate::util::unid::generate(),
            file_key: crate::util::file_key::generate(),
            mimetype,
            filesize,
            // Default values
            width: 250,
            height: 250,
            title,
            description,
            created: Utc::now(),
            source_url,
            is_private,
            state: CardState::Pending,
        }
    }

    /// Insert an entity into the databases
    pub async fn insert(&self, pool: &DatabasePool) -> Result<(), DatabaseError> {
        let mut tx = pool.begin().await?;
        {
            sqlx::query(
                "INSERT INTO cards (
                id, file_key, title, description, 
                source_url, mimetype, filesize,
                width, height, is_private, state
              ) VALUES (
                ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?, ?
              )",
            )
            .bind(self.id)
            .bind(&self.file_key)
            .bind(&self.title)
            .bind(&self.description)
            .bind(&self.source_url)
            .bind(&self.mimetype)
            .bind(self.filesize)
            .bind(self.width)
            .bind(self.height)
            .bind(self.is_private)
            .bind(&self.state)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        tracing::info!("New card inserted: {}", &self.id);

        Ok(())
    }

    /// Fetch a [`Card`] by its numeric ID
    pub async fn fetch_by_id(id: u64, pool: &DatabasePool) -> Result<Option<Self>, DatabaseError> {
        let card: Option<Card> = sqlx::query_as("SELECT * FROM cards WHERE id = ?")
            .bind(id as i64)
            .fetch_optional(pool)
            .await?;

        Ok(card)
    }

    /// Remove a card from the database
    pub async fn remove(&self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Debug, serde::Serialize, sqlx::Type, derive_more::Display)]
#[serde(rename_all = "lowercase")]
pub enum CardState {
    #[sqlx(rename = "pending")]
    #[display("pending")]
    Pending,
    #[sqlx(rename = "processing")]
    #[display("processing")]
    Processing,
    #[sqlx(rename = "ready")]
    #[display("ready")]
    Ready,
    #[sqlx(rename = "failed")]
    #[display("failed")]
    Failed,
}
