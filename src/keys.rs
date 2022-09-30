use log::debug;
use rand::Rng;
use std::char;

use chrono::naive::NaiveDateTime;
use chrono::Utc;

const MAX_64: u64 = 13535000000000000;
const MIN_64: u64 = 218400000000000;
// delta = 13_316_600_000_000_000

// base62 conversion table
const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

pub struct Keys {}

impl Keys {
    pub fn routing_key() -> String {
        let now = Keys::now();

        let ts = now.timestamp_micros() as u64;
        let key = Self::to_base62(ts);

        debug!(
            "time now: {:?}, ts: {}, value: {} len: {}",
            now,
            ts,
            key,
            key.len()
        );

        let mut pad: String = Self::gen_random();

        pad.insert_str(6, key.as_str());

        pad.to_string()
    }

    pub fn now() -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    pub fn get_timestamp(dt: NaiveDateTime) -> u64 {
        dt.timestamp_micros() as u64
    }

    // convert the number to a b36 string; pad left to 11 chars
    fn to_base62(number: u64) -> String {
        let radix = ALPHA.len() as u64;
        let mut n = number;
        let mut base: Vec<char> = Vec::with_capacity(30);

        loop {
            let idx = (n % radix) as usize;
            base.push(ALPHA[idx]);

            n /= radix;

            if n < radix {
                break;
            }
        }

        base.iter().rev().collect::<String>()
    }

    fn gen_random() -> String {
        let mut rng = rand::thread_rng();
        Self::to_base62(rng.gen_range(MIN_64..MAX_64))
    }

    pub fn decode_digit(digit: u8) -> u8 {
        const ZERO: u8 = 48;
        const NINE: u8 = 57;
        const BIG_A: u8 = 65;
        const BIG_Z: u8 = 65 + 26;
        const LITTLE_A: u8 = 97;
        const LITTLE_Z: u8 = 97 + 26;

        match digit {
            ZERO..=NINE => digit - ZERO,
            BIG_A..=BIG_Z => digit - BIG_A + 10,
            LITTLE_A..=LITTLE_Z => digit - LITTLE_A + 36,
            _ => panic!("out of range"),
        }
    }

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
    fn base62() {
        let b62 = Keys::to_base62(MIN_64);
        assert_eq!(b62, "013NOrRI");

        let b62 = Keys::to_base62(MAX_64);
        assert_eq!(b62, "zPGRMW7E");

        assert!(true)
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

    #[test]
    fn decode_digit() {
        let mut n = 0_u8;

        for x in b'0'..=b'9' {
            let y = Keys::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        for x in b'A'..=b'Z' {
            let y = Keys::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        for x in b'a'..=b'z' {
            let y = Keys::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        assert_eq!(n, 62);

        n = 0;
        for ch in ALPHA {
            let x = ch as u8;
            let y = Keys::decode_digit(x);
            assert_eq!(y, n);

            n += 1;
        }
    }
}
