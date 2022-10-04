use domain_keys::keys::RouteKey;
use std::collections::HashSet;

#[test]
fn unique_test() {
    const ROUTE_KEY_SIZE: usize = 16;

    let max_tests: usize = 10_000; // 10_000_000 <- do this in special integration tests;
    let mut table = HashSet::with_capacity(max_tests);

    for _ in 0..max_tests {
        let key = RouteKey::create();
        assert_eq!(key.len(), ROUTE_KEY_SIZE);
        assert_eq!(table.insert(key), true);
    }

    assert_eq!(table.len(), max_tests);
}

#[test]
fn parse_route() {
    // known Key: YM6I7clU96YvDTCr, TimeStamp: 1664899323738819
    let key = "YM6I7clU96YvDTCr".to_string();
    let total_routes = 25;

    if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
        assert_eq!(route, 5);
    } else {
        panic!("parse error for route, key: {}", key);
    }
}

#[test]
fn parse_timestamp() {
    let key = "YM6I7clU96YvDTCr".to_string();
    let ref_ts = 1664899323738819_u64;

    if let Ok(ts) = RouteKey::parse_timestamp(&key) {
        assert_eq!(ts, ref_ts);
    }
}
