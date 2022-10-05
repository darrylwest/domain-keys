use domain_keys::keys::RouteKey;

fn main() {
    let total_routes = 12;
    println!("Total Routes: {}", total_routes);
    for _ in 0..total_routes {
        let key = RouteKey::create();
        if let Ok(route) = RouteKey::parse_route(&key, total_routes) {
            println!("key: {} - route: {}", key, route);
        } else {
            panic!("broken key: {}", key);
        }
        assert_eq!(key.len(), 16);
    }
}
