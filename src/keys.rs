use crate::base62::Base62;
use rand::Rng;

use std::time::{SystemTime, UNIX_EPOCH};

const MAX_64: u64 = 3_521_614_606_207; // largest 7 digit from -> zzzzzzz
const MIN_64: u64 = 14_776_336; // smallest 5 digit conversionn from -> 0010000
const INSERT_INDEX: usize = 6;
const ROUTE_KEY_SIZE: usize = 16;

/// Define the micro timestamp
type NanoTimeStamp = u128;

pub enum KeysError<'a> {
    InvalidSize,
    InvalidBase62(&'a str),
}

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
        // get the timestamp in micros
        let ts = (Keys::now() / 1_000) as u64;
        let key = Base62::encode(ts);

        // now the random number padded to 7 chars
        let mut pad: String = Self::encode_with_pad(Self::gen_random());

        // insert the timestamp at the 6th position
        pad.insert_str(INSERT_INDEX, key.as_str());

        assert_eq!(pad.len(), ROUTE_KEY_SIZE);

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
    fn gen_random() -> u64 {
        let mut rng = rand::thread_rng();

        rng.gen_range(MIN_64..MAX_64)
    }

    // ensure 7 characters, padded with zeros...
    fn encode_with_pad(n: u64) -> String {
        format!("{:0>7}", Base62::encode(n))
    }

    /// Parse and return the route from the key's first two chars based on the total number of routes
    /// 
    pub fn get_route(key: &str, total_routes: u8) -> Result<u8, KeysError> {
        if key.len() < 2 {
            return Err(KeysError::InvalidSize);
        }

        let mut s = key.to_string();
        s.truncate(2);

        if let Ok(n) = Base62::decode(&s) {
            let route = (n % total_routes as u64) as u8;
            Ok(route)
        } else {
            Err(KeysError::InvalidBase62(key))
        }

    }

    // TODO implement calc_route from the key (first two digits) and the number of routes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn random_number_in_range() {
        for _ in 0..10 {
            assert!(Keys::gen_random() >= MIN_64);
            assert!(Keys::gen_random() <= MAX_64);
        }
    }

    #[test]
    fn get_route_10() {
        // test for 10 routes
        let total_routes = 10_u8;

        // create fake keys between 00 and zz
        let keys: Vec<String> = (0..3843_u64).into_iter()
            .map(|n| Base62::encode(n))
            .map(|s| format!("{:0>2}", s))
            .collect();
    
        let mut current = 0_u8;

        for key in keys {
            if let Ok(route) = Keys::get_route(&key, total_routes) {
                assert!(route < total_routes);
                assert_eq!(route, current);
                current = (current + 1) % total_routes;
            } else {
                panic!("could not get route for key: {:?}", key);
            }
        }
    }

    #[test]
    fn encode_padding_size() {
        // test max, min and halfway point
        [MAX_64, MIN_64, MAX_64 / 2]
            .iter()
            .map(|x| Keys::encode_with_pad(*x))
            .for_each(|s| assert_eq!(s.len(), 7));
    
        // test the formats for min and max
        assert_eq!(Keys::encode_with_pad(MIN_64), "0010000");
        assert_eq!(Keys::encode_with_pad(MAX_64), "zzzzzzz");
    }

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
