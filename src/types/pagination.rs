pub struct PaginationParams {
    page: u32,
    limit: u32,
}

impl PaginationParams {
    pub fn new(page: u32, limit: u32) -> Self {
        Self { page, limit }
    }

    pub fn page(&self) -> u32 {
        self.page
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn offset(&self) -> u32 {
        self.limit * self.page
    }
}
