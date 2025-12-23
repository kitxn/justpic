#[derive(Debug, sqlx::Type, serde::Serialize, utoipa::ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileState {
    #[sqlx(rename = "pending")]
    Pending,

    #[sqlx(rename = "processing")]
    Processing,

    #[sqlx(rename = "ready")]
    Ready,

    #[sqlx(rename = "failed")]
    Failed,
}

impl FileState {
    pub fn is_pending(&self) -> bool {
        matches!(self, FileState::Pending)
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, FileState::Processing)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, FileState::Ready)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, FileState::Failed)
    }

    pub fn is_processed(&self) -> bool {
        self.is_failed() | self.is_ready()
    }
}
