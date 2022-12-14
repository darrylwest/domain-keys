//!
//! txkey - generate a timestamp key, 12 chars 9 for the timestamp in micros
//! and 3 random.
//!
//! # Examples:
//!
//! ```bash
//! txkey
//! Key: 7coWCdVsNZjI, TimeStamp: 1665071608893359
//!
//! txkey -q
//! 7coWEn0Efg2p
//! ```
//!
use clap::Parser;
use domain_keys::keys::TimeStampKey;

#[derive(Debug, Default, Parser)]
#[clap(name = "txkey")]
#[command(author)]
#[clap(version = "0.7.1")]
#[clap(long_about = None)]
#[clap(about = "txkey\n\ngenerate a new txkey and write the key and timestamp to stdout.")]
pub struct CliArgs {
    /// set verbose to show the timestamp with the key
    #[clap(short, long, value_parser)]
    pub verbose: bool,
}

fn main() {
    let args = CliArgs::parse();

    let key = TimeStampKey::create();

    if !args.verbose {
        println!("{}", key);
    } else if let Ok(ts) = TimeStampKey::parse_timestamp(&key) {
        println!("Key: {}, TimeStamp: {}", key, ts);
    } else {
        println!("Key: {}, TimeStamp: ERROR", key);
    }
}
