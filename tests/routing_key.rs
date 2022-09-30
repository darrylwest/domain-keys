use domain_keys::keys::Keys;
use std::collections::HashSet;

#[test]

fn unique_test() {
    const ROUTE_KEY_SIZE: usize = 16;

    let max_tests: usize = 10_000; // 10_000_000 <- do this in special integration tests;
    let mut table = HashSet::with_capacity(max_tests);

    for _ in 0..max_tests {
        let key = Keys::routing_key();
        assert_eq!(key.len(), ROUTE_KEY_SIZE);
        assert_eq!(table.insert(key), true);
    }

    assert_eq!(table.len(), max_tests);
}
