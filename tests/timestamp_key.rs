use domain_keys::keys::TxKey;
use std::collections::HashSet;

#[test]
fn unique_test() {
    let max_tests: usize = 10_000; // 10_000_000 <- do this in special integration tests;
    let mut table = HashSet::with_capacity(max_tests);

    for _ in 0..max_tests {
        let key = TxKey::create();
        assert_eq!(key.len(), 12);
        assert_eq!(table.insert(key), true);
    }

    assert_eq!(table.len(), max_tests);
}

// parse timestamp
