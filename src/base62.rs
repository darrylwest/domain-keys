/*!
 * This is a doc...
 */

// base62 conversion table
const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

/// Empty string and invalid chars; used in decode.
#[derive(Debug)]
pub enum Base62Error {
    EmptyString,
    InvalidChar,
}

/// The magic starts here... an empty struct.
#[derive(Debug)]
pub struct Base62 {}

impl Base62 {
    ///
    /// Encode the given u64 value to Base62 using the character set 0..9, A..Z, a..z as specified [here](https://en.wikipedia.org/wiki/Base62).
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::base62::Base62;
    ///
    /// // some nano second timestamp from 2022-10-01
    /// let nanos: u64 = 1664650548820248432;
    /// let base62 = Base62::encode(nanos);
    ///
    /// assert_eq!(base62, "1yy7GPuHalc");
    /// assert_eq!(base62.len(), 11);
    ///
    /// // some micro second timestamp from 2022-10-01
    /// let micros: u64 = 1664650548820248;
    /// let base62 = Base62::encode(micros);
    ///
    /// assert_eq!(base62, "7ch6b4MAa");
    /// assert_eq!(base62.len(), 9);
    ///
    /// // some micro second timestamp from 2390-01-01
    /// let micros: u64 = 13253932800000000;
    /// let base62 = Base62::encode(micros);
    ///
    /// assert_eq!(base62, "yhav0arrM");
    ///
    /// assert_eq!(Base62::encode(0), "0");
    /// assert_eq!(Base62::encode(10), "A");
    /// assert_eq!(Base62::encode(61), "z");
    /// assert_eq!(Base62::encode(62), "10");
    /// assert_eq!(Base62::encode(63), "11");
    ///
    /// ```
    ///
    pub fn encode(number: u64) -> String {
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

    ///
    /// Decode the base62 string and return Result<u64, Base62Error>. Checks for empty string and invalid char.  
    ///
    /// # Example:
    ///
    /// ```rust
    /// use domain_keys::base62::Base62;
    ///
    /// let b62 = "zZAa";
    /// let value = Base62::decode(&b62.to_string()).expect("should get a number");
    ///
    /// assert_eq!(value, 14673204);
    ///
    /// // test for empty string error
    /// assert!(Base62::decode(&"".to_string()).is_err());
    ///
    /// ```
    pub fn decode(b62: &str) -> Result<u64, Base62Error> {
        // validate string is not empty
        if b62.is_empty() {
            return Err(Base62Error::EmptyString);
        }

        // validate chars in the string

        let radix = 62_u64;
        let mut result = 0_u64;

        for (p, ch) in b62.chars().rev().enumerate() {
            let n = Self::decode_digit(ch as u8) as u64;
            let q = radix.pow(p as u32);

            result += n * q;
        }

        Ok(result)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_encode() {
        let value = 1000_u64;
        let b62 = Base62::encode(value);
        assert_eq!(b62, "G8".to_string());
    }

    #[test]
    fn simple_decode() {
        let b62 = String::from("zaZA90");
        let n = Base62::decode(&b62.to_string()).unwrap();
        assert_eq!(n, 56424431326);
    }

    #[test]
    fn decode_empty_string() {
        let b62 = String::from("");
        match Base62::decode(&b62.to_string()) {
            Ok(_) => panic!("should not be ok"),
            Err(err) => println!("err: {:?}", err),
        }

        assert!(Base62::decode(&"".to_string()).is_err());
    }

    #[test]
    fn decode_invalid_char() {
        // TODO...
        assert!(true);
    }

    #[test]
    fn base62_correctness() {
        // TODO replace with 0..9, A..Z, a..z; use decode to verify
        assert_eq!(Base62::encode(0), "0");
        assert_eq!(Base62::encode(9), "9");
        assert_eq!(Base62::encode(10), "A");
        assert_eq!(Base62::encode(35), "Z");
        assert_eq!(Base62::encode(36), "a");
        assert_eq!(Base62::encode(61), "z");
        assert_eq!(Base62::encode(62), "10");
        assert_eq!(Base62::encode(63), "11");

        let n = u64::MAX;
        let b62 = "LygHa16AHYF".to_string();
        assert_eq!(Base62::encode(n), b62);
        assert_eq!(b62.len(), 11);
    }

    #[test]
    fn decode_digit() {
        let mut n = 0_u8;

        for x in b'0'..=b'9' {
            let y = Base62::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        for x in b'A'..=b'Z' {
            let y = Base62::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        for x in b'a'..=b'z' {
            let y = Base62::decode_digit(x);
            assert_eq!(y, n);
            n += 1;
        }

        assert_eq!(n, 62);

        n = 0;
        for ch in ALPHA {
            let x = ch as u8;
            let y = Base62::decode_digit(x);
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
            let value = Base62::decode(&b62.to_string()).unwrap();
            assert_eq!(n, value);
        }
    }
}
