use clap::Parser;
use domain_keys::keys::Keys;
// use domain_keys::config::Config;

#[derive(Debug, Default, Parser)]
#[clap(name = "rtkey")]
#[command(author)]
#[clap(version = "1.3")]
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

/// The CLI `rtkeys` generates or decodes a routing key.
///
/// # Example:
///
fn main() {
    // let config = Config::new();
    let args = CliArgs::new();

    let key = Keys::routing_key();

    if args.quiet {
        println!("{}", key);
    } else {
        println!("Key: {}, TimeStamp: {}", key, 0);
    }

}
