use crate::{
    error::Error,
    models::{
        cards::internal::Card,
        files::{internal::File, uploading::UploadedFile},
    },
    utils::multipart::{MultipartSegment, MultipartSegmentItem},
};

pub struct MultipartCardBuilder {
    file: Option<UploadedFile>,

    title: Option<String>,

    description: Option<String>,

    source_url: Option<String>,

    is_private: bool,
}

impl MultipartCardBuilder {
    pub fn new() -> Self {
        MultipartCardBuilder {
            file: None,
            title: None,
            description: None,
            source_url: None,
            is_private: false,
        }
    }

    pub fn set_file(&mut self, file: UploadedFile) {
        self.file = Some(file);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_source_url(&mut self, source_url: String) {
        self.source_url = Some(source_url);
    }

    pub fn set_is_private(&mut self, is_private: bool) {
        self.is_private = is_private;
    }

    pub fn handle_segment(&mut self, seg: MultipartSegment) {
        match (seg.key(), seg.value()) {
            ("title", MultipartSegmentItem::Text(t)) => {
                self.set_title(t.clone());
            }
            ("description", MultipartSegmentItem::Text(t)) => {
                self.set_description(t.clone());
            }
            ("is_private", MultipartSegmentItem::Text(t)) => {
                self.set_is_private(t == "true");
            }
            ("file", MultipartSegmentItem::File(f)) => {
                self.set_file(f.clone());
            }
            _ => {}
        };
    }

    pub fn build(self, owner_id: uuid::Uuid) -> Result<Card, Error> {
        let file = self.file.ok_or(Error::BadInput)?;

        let file = File::from_uploaded(file, owner_id);
        let card = Card::new(
            file,
            owner_id,
            self.title,
            self.description,
            self.source_url,
            self.is_private,
        );

        Ok(card)
    }
}

impl Default for MultipartCardBuilder {
    fn default() -> Self {
        Self::new()
    }
}
