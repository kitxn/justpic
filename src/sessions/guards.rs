use crate::{error::Error, sessions::models::Session};

/// Returns an error if given an active session
pub fn except_active_session(session: Option<Session>) -> Result<(), Error> {
    if session.is_some_and(|v| !v.is_expired()) {
        return Err(Error::Conflict);
    }

    Ok(())
}

/// Returns an error if it receives an expired session.
pub fn except_expired_session(session: &Session) -> Result<(), Error> {
    if session.is_expired() {
        return Err(Error::Unauthorized);
    }

    Ok(())
}
