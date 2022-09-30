
use domain_keys::keys::Keys;

fn main() {
    let key = Keys::routing_key();
    println!("{}", key);
}

