use domain_keys::keys::Keys;

fn main() {
    for n in 0..10 {
        let key = Keys::routing_key();
        println!("{} - {}", n, key);
        assert_eq!(key.len(), 16);
    }
}
