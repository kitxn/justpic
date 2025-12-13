#[utoipa::path(
  get, 
  path = "/files/{id}", 
  tag = "files", 
  params(
    (
      "id", 
      description = "File ID", 
      example = "0c7a8e9fcc98496cb75b64b51d6aedf1"
    ),
  )
)]
pub fn get_file_stream() {}
