use domain_keys::base62::Base62;

fn generate_test_data() -> Vec<(u64, &'static str)> {
    let mut tdata = vec![];

    tdata.push((0, "0"));
    tdata.push((1, "1"));
    tdata.push((9, "9"));
    tdata.push((10, "A"));
    tdata.push((35, "Z"));
    tdata.push((36, "a"));
    tdata.push((61, "z"));
    tdata.push((1_000, "G8"));
    tdata.push((1_000_000, "4C92"));
    tdata.push((u64::MAX, "LygHa16AHYF"));

    tdata
}

#[test]
fn encode() {
    for (value, expected) in generate_test_data() {
        let b62 = Base62::encode(value);
        assert_eq!(b62, expected);
    }
}

#[test]
fn decode() {
    for (expected, value) in generate_test_data() {
        let decoded = Base62::decode(value).expect("should decode");

        assert_eq!(decoded, expected);
    }
}

#[test]
fn encode_decode() {
    for n in 0..100 {
        let b62 = Base62::encode(n);
        let decoded = Base62::decode(&b62).expect("should decode base62");

        assert_eq!(n, decoded);
    }
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
    let b62 = String::from("AB~CY");
    match Base62::decode(&b62.to_string()) {
        Ok(_) => panic!("should not be ok"),
        Err(err) => println!("err: {:?}", err),
    }

    assert!(Base62::decode(&"LLLL&GG".to_string()).is_err());
    assert!(Base62::decode(&"-bad".to_string()).is_err());
}
