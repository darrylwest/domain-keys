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
#[clap(version = "1.4.0")]
#[clap(long_about = None)]
#[clap(about = "rtkey\n\ngenerate a new key and write the key and timestamp to stdout.")]
pub struct CliArgs {
    /// set quiet to suppress the timestamp and show only the key
    #[clap(short, long, value_parser)]
    pub quiet: bool,
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

    if args.quiet {
        println!("{}", key);
    } else if let Ok(ts) = RouteKey::parse_timestamp(&key) {
        println!("Key: {}, TimeStamp: {}", key, ts);
    } else {
        println!("Key: {}, TimeStamp: ERROR", key);
    }
}
