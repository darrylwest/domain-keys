use domain_keys::keys::RouteKey;

fn main() {
    for n in 0..10 {
        let key = RouteKey::create();
        println!("{} - {}", n, key);
        assert_eq!(key.len(), 16);
    }
}
