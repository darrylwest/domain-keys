use domain_keys::keys::RouteKey;

fn main() {
    let total_routes = 8;
    println!("Total Routes: {}", total_routes);
    for _ in 0..50 {
        let key = RouteKey::create();
        if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
            println!("route: {} key: {}", route, key);
        } else {
            panic!("broken key: {}", key);
        }
        assert_eq!(key.len(), 16);
    }
}
