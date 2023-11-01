//! The CLI `rtkeys` generates or decodes a routing key.
//!
//! # Examples:
//!
//! ```bash
//! rtkey
//! Key: hhKZ7coVyViUJNCM, TimeStamp: 1665071400166511
//!
//!
//! rtkey -q
//! iGqc7coW37bHH8AY
//! ```
//!
use clap::Parser;
use domain_keys::keys::RouteKey;

#[derive(Debug, Default, Parser)]
#[clap(name = "rtkey")]
#[command(author)]
#[clap(version = "0.7.1")]
#[clap(long_about = None)]
#[clap(
    about = "rtkey\n\ngenerate new key(s) and write to stdout. include timestamp when creating a single key."
)]
pub struct CliArgs {
    /// set verbose to show the timestamp with key
    #[clap(short, long, value_parser)]
    pub verbose: bool,

    #[clap(short, long, value_parser, default_value = "1")]
    pub count: u16,
}

impl CliArgs {
    pub fn new() -> Self {
        Self::parse()
    }
}

fn main() {
    // let config = Config::new();
    let args = CliArgs::new();

    let key = RouteKey::create();

    assert_eq!(key.len(), 16);

    if args.count > 1 {
        print!("{} ", key);
        for _ in 1..args.count {
            print!("{} ", RouteKey::create());
        }
        println!();
    } else if !args.verbose {
        println!("{}", key);
    } else if let Ok(ts) = RouteKey::parse_timestamp(&key) {
        println!("Key: {}, TimeStamp: {}", key, ts);
    } else {
        println!("Key: {}, TimeStamp: ERROR", key);
    }
}
