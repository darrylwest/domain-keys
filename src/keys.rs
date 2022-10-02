use rand::Rng;
use std::char;

use std::time::{SystemTime, UNIX_EPOCH};

const MAX_64: u64 = 3521500000000;
const MIN_64: u64 = 56810000000;
// delta = 3_464_804_000_000

// base62 conversion table
const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

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

        let key = Self::to_base62(ts);

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

    /// convert the u64 number to a base 62 string.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::keys::Keys;
    ///
    /// // some nano second timestamp from 2022-10-01
    /// let nanos: u64 = 1664650548820248432;
    /// let base62 = Keys::to_base62(nanos);
    ///
    /// assert_eq!(base62, "1yy7GPuHalc");
    /// assert_eq!(base62.len(), 11);
    ///
    /// // some micro second timestamp from 2022-10-01
    /// let micros: u64 = 1664650548820248;
    /// let base62 = Keys::to_base62(micros);
    ///
    /// assert_eq!(base62, "7ch6b4MAa");
    /// assert_eq!(base62.len(), 9);
    ///
    /// // some micro second timestamp from 2390-01-01
    /// let micros: u64 = 13253932800000000;
    /// let base62 = Keys::to_base62(micros);
    ///
    /// assert_eq!(base62, "yhav0arrM");
    ///
    /// assert_eq!(Keys::to_base62(0), "0");
    /// assert_eq!(Keys::to_base62(10), "A");
    /// assert_eq!(Keys::to_base62(61), "z");
    /// assert_eq!(Keys::to_base62(62), "10");
    /// assert_eq!(Keys::to_base62(63), "11");
    ///
    /// ```
    ///
    pub fn to_base62(number: u64) -> String {
        let radix = ALPHA.len() as u64;
        let mut n = number;
        let mut base: Vec<char> = Vec::with_capacity(16);

        loop {
            let idx = (n % radix) as usize;
            base.push(ALPHA[idx]);

            n /= radix;

            if n == 0 {
                break;
            }
        }

        base.iter().rev().collect::<String>()
    }

    pub fn from_base62(b62: &str) -> u64 {
        let radix = 62_u64;
        let mut result = 0_u64;

        for (p, ch) in b62.chars().rev().enumerate() {
            let n = Self::decode_digit(ch as u8) as u64;
            let q = radix.pow(p as u32);

            result += n * q;
        }

        result
    }

    fn decode_digit(digit: u8) -> u8 {
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

    // return a random number between min and max to stay in the 7 character range
    fn gen_random() -> String {
        let mut rng = rand::thread_rng();
        Self::to_base62(rng.gen_range(MIN_64..MAX_64))
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
    fn base62_correctness() {
        assert_eq!(Keys::to_base62(0), "0");
        assert_eq!(Keys::to_base62(9), "9");
        assert_eq!(Keys::to_base62(10), "A");
        assert_eq!(Keys::to_base62(35), "Z");
        assert_eq!(Keys::to_base62(36), "a");
        assert_eq!(Keys::to_base62(61), "z");
        assert_eq!(Keys::to_base62(62), "10");
        assert_eq!(Keys::to_base62(63), "11");

        let n = u64::MAX;
        let b62 = "LygHa16AHYF".to_string();
        assert_eq!(Keys::to_base62(n), b62);
        assert_eq!(b62.len(), 11);
    }

    #[test]
    fn base62_key_limits() {
        let b62 = Keys::to_base62(MIN_64);
        assert_eq!(b62, "100eyAa");
        assert_eq!(b62.len(), 7);

        let b62 = Keys::to_base62(MAX_64);
        assert_eq!(b62.len(), 7);
        assert_eq!(b62, "zzsF7gm");

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

    #[test]
    fn decode_base62() {
        let list = [
            ("0", 0),
            ("9", 9),
            ("A", 10),
            ("Z", 35),
            ("a", 36),
            ("z", 61),
            ("10", 62),
            ("11", 63),
        ];

        for (b62, n) in list {
            assert_eq!(n, Keys::from_base62(&b62.to_string()));
        }
    }
}
