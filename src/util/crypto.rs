use bcrypt::DEFAULT_COST;

use crate::error::Result;

/// Hash given value via bcrypt
pub fn bcrypt_hash(value: &str) -> Result<String> {
    let hash = bcrypt::hash(value, DEFAULT_COST)?;
    Ok(hash)
}

/// Check if the value matches the hash
pub fn bcrypt_validate(value: &str, hashed: &str) -> Result<bool> {
    let valid = bcrypt::verify(value, hashed)?;
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let pass = "hunter42!";

        let hash = bcrypt_hash(pass).unwrap();

        assert_ne!(hash, pass);
    }

    #[test]
    fn test_hash_validation() {
        let pass = "hunter42!";

        let hash = bcrypt_hash(pass).unwrap();

        assert!(bcrypt_validate(pass, &hash).unwrap());
        assert!(!bcrypt_validate("hunter52", &hash).unwrap());
    }
}
