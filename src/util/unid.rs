//! Unique Number ID

use rand::Rng;

/// Generate random number id
/// (like `4955151763728138`)
pub fn generate() -> i64 {
    let timestamp = chrono::Utc::now().timestamp_millis();

    let rng = rand::rng().random_range(10000..99999) as i64;

    rng * 10i64.pow(timestamp.ilog10() + 1) + timestamp
}

#[cfg(test)]
mod tests {
    use crate::util::unid::generate;

    #[test]
    fn test_unid_generation() {
        let id = generate();

        // Checking that the id length is 16
        assert_eq!(id.ilog10() + 1, 18);

        // Checking that the ids are not the same
        let id2 = generate();
        assert_ne!(id, id2);

        // Checking that an id is not just a timestamp
        let timestamp = chrono::Utc::now().timestamp();
        assert_ne!(id, timestamp);
    }
}
