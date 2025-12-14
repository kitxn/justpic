use crate::models::users::responses::common::UserPublic;

use super::role::UserRole;

/// Internal model for the user entity
#[derive(sqlx::FromRow)]
pub struct User {
    id: uuid::Uuid,

    username: String,
    password: String,

    role: UserRole,

    created: chrono::DateTime<chrono::Utc>,
}

impl User {
    // -- Getters --
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn id_copy(&self) -> uuid::Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn created(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created
    }

    // -- Features --
    pub fn new(username: &str, password_hash: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            username: username.to_lowercase(),
            password: password_hash,
            role: UserRole::Regular,
            created: chrono::Utc::now(),
        }
    }

    /// Is the user an administrator?
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    // -- Mappers --
    /// Make the entity public by removing sensitive fields
    pub fn to_public_model(self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: self.username,
            role: self.role,
            created: self.created,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_user_for_test() -> User {
        User::new("john_doe", "hunter42!".to_string())
    }

    fn create_admin_user_for_test() -> User {
        let mut item = create_user_for_test();
        item.role = UserRole::Admin;

        item
    }

    #[test]
    fn test_user_model_creation() {
        let item = create_user_for_test();

        assert!(!item.is_admin(), "New user cannot be admin");

        assert_ne!(
            item.created().timestamp(),
            0,
            "The new user must have a non-zero creation date."
        );

        assert_eq!(
            item.username(),
            "john_doe",
            "The new user name must match the passed argument."
        );
    }

    #[test]
    fn test_to_public_model_method() {
        let item = create_user_for_test();

        let id = item.id;

        let public_model = item.to_public_model();
        assert_eq!(
            public_model.username.as_str(),
            "john_doe",
            "The public username must match the original username"
        );

        assert_eq!(
            &public_model.id, &id,
            "The identifiers for the user and its public version must match"
        );
    }

    #[test]
    fn test_user_admin_checking() {
        let not_admin = create_user_for_test();
        let admin = create_admin_user_for_test();

        assert!(
            !not_admin.is_admin(),
            "For a regular user, the admin role check should return FALSE"
        );

        assert!(
            admin.is_admin(),
            "For an administrator, the admin role check should return TRUE"
        )
    }
}
