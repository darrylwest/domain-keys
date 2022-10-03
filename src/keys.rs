use crate::base62::Base62;
use rand::Rng;

use std::time::{SystemTime, UNIX_EPOCH};

const MAX_64: u64 = 3521614606207; // largest 7 digit from -> zzzzzzz
const MIN_64: u64 = 14776336; // smallest 5 digit conversionn from -> 10000
                              // delta = 3_464_804_000_000

/// Define the micro timestamp
type NanoTimeStamp = u128;

pub struct Keys {}

impl Keys {
    /// Generate a new base62 routing key.
    ///
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::Keys;
    ///
    /// let key = Keys::routing_key();
    ///
    /// assert_eq!(key.len(), 16);
    /// ```
    pub fn routing_key() -> String {
        let ts = (Keys::now() / 1_000) as u64;

        let key = Base62::encode(ts);

        let mut pad: String = Self::gen_random();

        pad.insert_str(6, key.as_str());

        pad.to_string()
    }

    /// return the current time in nanoseconds.  time is from the system clock.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::Keys;
    ///
    /// let t0 = Keys::now();
    /// if std::env::consts::OS == "macos" {
    ///     println!("NOTE: mac os only resolves to microseconds...");
    /// }
    /// let t1 = Keys::now();
    ///
    /// assert!(t0 < t1);
    /// ```
    /// NOTE: osx time defaults to microseconds but the NanoTimeStamp is looking for nanos.  This shouldn't matter
    /// for routing keys because they always resolve to micros.  But if you plan to use time based `txkey` with
    /// nano seconds, be aware that mac always trucates the nanos to `000`.  Not a problem on linux.
    ///
    pub fn now() -> NanoTimeStamp {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => t.as_nanos(),
            Err(_) => panic!("System time befor Unix Epoch"),
        }
    }

    // return a random number between min and max to stay in the 7 character range
    fn gen_random() -> String {
        let mut rng = rand::thread_rng();

        // make sure we always return 7 characters, padded with zeros...
        format!("{:07}", Base62::encode(rng.gen_range(MIN_64..MAX_64)))
    }

    /*
    pub fn get_route(&self, total_routes: u8) -> u8 {
        (n % total_routes) as u8
    }
    */

    // TODO implement calc_route from the key (first two digits) and the number of routes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const ROUTE_KEY_SIZE: usize = 16;

    #[test]
    fn routing_key() {
        let key = Keys::routing_key();

        assert_eq!(key.len(), ROUTE_KEY_SIZE);
    }

    #[test]
    fn unique_test() {
        // this is hardly exhaustive; there more thorough integration tests...
        let max_tests: usize = 1_000;
        let mut table = HashSet::with_capacity(max_tests);

        for _ in 0..max_tests {
            let key = Keys::routing_key();
            assert_eq!(key.len(), ROUTE_KEY_SIZE);
            assert_eq!(table.insert(key), true);
        }

        assert_eq!(table.len(), max_tests);
    }
}
