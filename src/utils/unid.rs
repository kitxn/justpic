//! Unique Number ID

use rand::Rng;

/// The number of least significant bits that will be filled random
const TIMESTAMP_PAD_OFFSET: u8 = 28;

/// Generate random number id
/// (like `12857460700869832838`)
pub fn generate() -> u64 {
    let ts = chrono::Utc::now().timestamp_millis();

    generate_from_timestamp(ts)
}

fn generate_from_timestamp(ts: i64) -> u64 {
    let ts_part = (ts as u64) << TIMESTAMP_PAD_OFFSET;

    let rn = rand::rng().random::<u32>();
    let rn_part = (rn as u64) >> 4;

    ts_part | rn_part
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const COLLISION_TEST_ITERATIONS: usize = 16_000;

    #[test]
    fn test_id_collisions_by_random_timestamp() {
        let mut rnd = rand::rng();

        let ids = (0..COLLISION_TEST_ITERATIONS)
            .map(|_| {
                let ts = rnd.random::<i64>();
                generate_from_timestamp(ts)
            })
            .collect::<HashSet<_>>();

        assert_eq!(
            ids.len(),
            COLLISION_TEST_ITERATIONS,
            "An id collision occurred"
        )
    }

    #[test]
    fn test_id_collisions_with_one_timestamp() {
        let fake_ts = 1765742986_i64;

        let ids = (0..COLLISION_TEST_ITERATIONS)
            .map(|_| generate_from_timestamp(fake_ts))
            .collect::<HashSet<_>>();

        assert_eq!(
            ids.len(),
            COLLISION_TEST_ITERATIONS,
            "An id collision occurred"
        )
    }

    #[test]
    fn test_id_length() {
        let id = generate();

        assert!(
            (16..19).contains(&id.to_string().len()),
            "The identifier length must be between 16 and 19"
        );
    }
}
