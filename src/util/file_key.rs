//! Unique file key

/// Generate random file key
pub fn generate() -> String {
    let uuid = uuid::Uuid::new_v4().to_string();

    uuid.replace("-", "")
}

#[cfg(test)]
mod tests {
    use crate::util::file_key;

    #[test]
    fn test_file_key_generation() {
        let key = file_key::generate();

        assert_eq!(key.len(), 32);
    }
}
