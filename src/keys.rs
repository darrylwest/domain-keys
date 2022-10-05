use crate::base62::Base62;
use rand::Rng;

use std::time::{SystemTime, UNIX_EPOCH};

const MAX_64: u64 = 3_521_614_606_207; // largest 7 digit from -> zzzzzzz
const MIN_64: u64 = 14_776_336; // smallest 5 digit conversionn from -> 0010000
const INSERT_INDEX: usize = 4;
const ROUTE_KEY_SIZE: usize = 16;

/// Define the micro timestamp
type NanoTimeStamp = u128;

#[derive(Debug)]
pub enum DomainKeyError<'a> {
    InvalidSize,
    InvalidBase62(&'a str),
    ParseError,
}

pub struct RouteKey {}

impl RouteKey {
    /// Generate a new base62 routing key.
    ///
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::RouteKey;
    ///
    /// let key = RouteKey::create();
    ///
    /// assert_eq!(key.len(), 16);
    /// ```
    pub fn create() -> String {
        // get the timestamp in micros
        let ts = (RouteKey::now() / 1_000) as u64;
        let key = Base62::encode(ts);

        // println!("ts: {}, enc: {}", ts, &key);

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
    /// use domain_keys::keys::RouteKey;
    ///
    /// let t0 = RouteKey::now();
    /// if std::env::consts::OS == "macos" {
    ///     println!("NOTE: mac os only resolves to microseconds...");
    /// }
    /// let t1 = RouteKey::now();
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

    /// Parse and return the route from the key's first two chars based on the total number of routes specified.
    /// Total routes should be within 1..128 and the input is silentlyt clamped to that range.
    /// The key should be a standard routing key, but since we just need the first two characters the lenth check is for 2.
    /// If the length is < 2 a RouteKeyError is returned.
    ///
    /// Routes are returned as a u8 in the range of 0..total_routes.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::RouteKey;
    ///
    /// let key = RouteKey::create();
    ///
    /// if let Ok(route) = RouteKey::parse_route(&key, 1) {
    ///     assert_eq!(route, 0); // a single route always returns route# 0
    /// } else {
    ///     panic!("bad route parse for key: {}", key);
    /// }
    ///
    /// let total_routes = 24_u8;
    /// if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
    ///     assert!((0..total_routes).contains(&route));
    /// } else {
    ///     panic!("bad route parse for key: {}", key);
    /// }
    ///
    /// let total_routes = 128_u8;
    /// if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
    ///     assert!((0..total_routes).contains(&route));
    /// } else {
    ///     panic!("bad route parse for key: {}", key);
    /// }
    ///
    /// // test the clamp to 128
    /// if let Ok(route) = RouteKey::parse_route(&key, 200_u8) {
    ///     assert!((0..total_routes).contains(&route));
    /// } else {
    ///     panic!("bad route parse for key: {}", key);
    /// }
    ///
    /// // test the error
    /// let bad_key = String::new();
    /// if let Ok(route) = RouteKey::parse_route(&bad_key, 1) {
    ///     panic!("this should have failed");
    /// }
    ///
    /// ```
    ///
    pub fn parse_route(key: &str, total_routes: u8) -> Result<u8, DomainKeyError> {
        if key.len() < 2 {
            return Err(DomainKeyError::InvalidSize);
        }
        let troutes = total_routes.clamp(1, 128);

        let mut s = key.to_string();
        s.truncate(2);

        if let Ok(n) = Base62::decode(&s) {
            let route = (n % troutes as u64) as u8;
            Ok(route)
        } else {
            Err(DomainKeyError::InvalidBase62(key))
        }
    }

    /// Parse the timestamp from the valid routing key.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::RouteKey;
    ///
    /// let now = RouteKey::now() as u64 / 1000_u64;
    /// let key = RouteKey::create();
    ///
    /// if let Ok(time_stamp) = RouteKey::parse_timestamp(&key) {
    ///     assert!(now <= time_stamp);
    /// } else {
    ///     panic!("parse time stamp failed for key: {}", key);
    /// }
    /// ```
    pub fn parse_timestamp(key: &str) -> Result<u64, DomainKeyError> {
        if key.len() != ROUTE_KEY_SIZE {
            return Err(DomainKeyError::InvalidSize);
        }

        // pull the timestamp from the key, always 8 chars
        let encoded_timestamp = &key[INSERT_INDEX..=INSERT_INDEX + 8];

        // println!("key: {}, enc ts: {} ", &key, &encoded_timestamp);

        if let Ok(ts) = Base62::decode(encoded_timestamp) {
            Ok(ts)
        } else {
            Err(DomainKeyError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn parse_timestamp() {
        let now = RouteKey::now() as u64 / 1000_u64;
        let key = RouteKey::create();

        if let Ok(ts) = RouteKey::parse_timestamp(&key) {
            assert!(ts >= now);
        } else {
            panic!("not a valid timestamp");
        }

        assert!(true);
    }

    #[test]
    fn parse_timestamp_error() {
        let key = "sxxskw".to_string();

        if let Ok(ts) = RouteKey::parse_timestamp(&key) {
            panic!("this key should fail: {} -> {}", &key, ts);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn random_number_in_range() {
        for _ in 0..10 {
            assert!(RouteKey::gen_random() >= MIN_64);
            assert!(RouteKey::gen_random() <= MAX_64);
        }
    }

    #[test]
    fn parse_route_25() {
        // test for 10 routes
        let total_routes = 25_u8;

        // create fake keys between 00 and zz
        let keys: Vec<String> = (0..3843_u64)
            .into_iter()
            .map(|n| Base62::encode(n))
            .map(|s| format!("{:0>2}", s))
            .collect();

        let mut current = 0_u8;

        for key in keys {
            if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
                assert!(route < total_routes);
                assert_eq!(route, current);
                current = (current + 1) % total_routes;
            } else {
                panic!("could not get route for key: {:?}", key);
            }
        }
    }

    #[test]
    fn parse_route_10() {
        // test for 10 routes
        let total_routes = 10_u8;

        // create fake keys between 00 and zz
        let keys: Vec<String> = (0..3843_u64)
            .into_iter()
            .map(|n| Base62::encode(n))
            .map(|s| format!("{:0>2}", s))
            .collect();

        let mut current = 0_u8;

        for key in keys {
            if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
                assert!(route < total_routes);
                assert_eq!(route, current);
                current = (current + 1) % total_routes;
            } else {
                panic!("could not get route for key: {:?}", key);
            }
        }
    }

    #[test]
    fn parse_route_from_key() {
        let key = RouteKey::create();

        let test_route = |total_routes| {
            if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
                // special case when there is only a single route
                if total_routes < 2 {
                    assert_eq!(route, 0);
                } else {
                    assert!(route < total_routes);
                }
            };
        };

        [1u8, 10u8, 24u8, 120u8].into_iter().for_each(test_route);
    }

    #[test]
    fn encode_padding_size() {
        // test max, min and halfway point
        [MAX_64, MIN_64, MAX_64 / 2]
            .iter()
            .map(|x| RouteKey::encode_with_pad(*x))
            .for_each(|s| assert_eq!(s.len(), 7));

        // test the formats for min and max
        assert_eq!(RouteKey::encode_with_pad(MIN_64), "0010000");
        assert_eq!(RouteKey::encode_with_pad(MAX_64), "zzzzzzz");
    }

    #[test]
    fn create() {
        let key = RouteKey::create();

        assert_eq!(key.len(), ROUTE_KEY_SIZE);
    }

    #[test]
    fn unique_test() {
        // this is hardly exhaustive; there more thorough integration tests...
        let max_tests: usize = 1_000;
        let mut table = HashSet::with_capacity(max_tests);

        for _ in 0..max_tests {
            let key = RouteKey::create();
            assert_eq!(key.len(), ROUTE_KEY_SIZE);
            assert_eq!(table.insert(key), true);
        }

        assert_eq!(table.len(), max_tests);
    }
}
