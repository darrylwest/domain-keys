use domain_keys::keys::Keys;
// use domain_keys::config::Config;

/// The CLI `rtkeys` generates or decodes a routing key.
///
/// # Example:
///
fn main() {
    // let config = Config::new();

    let key = Keys::routing_key();
    println!("{}", key);
}
