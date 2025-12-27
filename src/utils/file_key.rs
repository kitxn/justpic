//! Unique file key

/// Generate random file key
pub fn generate() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::file_key;

    #[test]
    fn test_file_key_generation() {
        let key = file_key::generate();

        assert_eq!(key.len(), 32);
    }
}
