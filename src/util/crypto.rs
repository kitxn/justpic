use bcrypt::DEFAULT_COST;

use crate::error::Error;

pub fn hash_password(value: &str) -> Result<String, Error> {
    let hash = bcrypt::hash(value, DEFAULT_COST)?;
    Ok(hash)
}

pub fn verify_password(value: &str, hash: &str) -> Result<bool, Error> {
    Ok(bcrypt::verify(value, hash)?)
}
