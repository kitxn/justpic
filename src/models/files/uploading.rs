#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub file_id: String,
    pub mimetype: String,
    pub size: usize,
}
