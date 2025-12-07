// CRUD MODULES
mod insert;

mod fetch_by_id;
mod fetch_by_username;

mod update_password;
mod update_username;

mod delete;

// PROVIDES
pub use insert::insert;

pub use fetch_by_id::fetch_by_id;
pub use fetch_by_username::fetch_by_username;

pub use update_password::change_password;
