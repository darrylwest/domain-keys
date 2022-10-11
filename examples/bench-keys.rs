use domain_keys::keys::RouteKey;
use std::time::Instant;

fn main() {
    let total_routes = 100_000_000_usize;
    println!("Total Routes: {}", total_routes);
    let now = Instant::now();
    for _ in 0..total_routes {
        let key = RouteKey::create();

        assert_eq!(key.len(), 16);
    }

    let elapsed = now.elapsed();

    println!("keys for {} routes took {} nano seconds", total_routes, elapsed.as_nanos());
}
