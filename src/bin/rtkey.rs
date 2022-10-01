use domain_keys::keys::Keys;
// use domain_keys::config::Config;

fn main() {
    // let config = Config::new();

    let key = Keys::routing_key();
    println!("{}", key);
}
