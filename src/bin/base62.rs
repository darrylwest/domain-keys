//!
//! Base62 encode and decode
//!
//! A command line utility to encode a u64 number to a base62 string with characters [0..9A..Za..z] or
//! to decode a base62 string to a u64 number.
//!

use clap::Parser;
use domain_keys::base62::Base62;

#[derive(Debug, Default, Parser)]
#[clap(name = "base62")]
#[command(author)]
#[clap(version = "1.3")]
#[clap(long_about = None)]
#[clap(about = "base62\n\nEncode a u64 number to base62, or decode a base62 String to u64.")]
pub struct CliArgs {
    /// encodes a u64 and outputs the string. `base62 -e 12345` -> 3d7
    #[clap(short, long, value_parser, value_name = "u64_number")]
    pub encode: Option<u64>,

    /// decodes a base62 encoded string [0..9A..Za..z] and outputs the integer.  `base62 -d 3d7` -> 12345
    #[clap(short, long, value_parser, value_name = "base62_string")]
    pub decode: Option<String>,

    /// encodes the current system UTC nano-second to base62
    #[clap(short, long, value_parser)]
    pub timestamp: bool,

    #[clap(short, long, value_parser)]
    pub quiet: bool,
}

impl CliArgs {
    pub fn new() -> Self {
        Self::parse()
    }
}

fn main() {
    let args = CliArgs::new();

    println!("{:?}", args);

    if let Some(n) = args.encode {
        println!("{}", Base62::encode(n));
    } else if let Some(s) = args.decode {
        match Base62::decode(&s) {
            Ok(n) => println!("{}", n),
            Err(err) => eprintln!("Error parsing Base62 string: {:?}", err),
        }
    } else if args.timestamp {
        let now = domain_keys::keys::Keys::now() as u64;
        println!("{} -> {}", now, Base62::encode(now));
    } else {
        println!("\nError: must add switch to --encode or --decode; try base62 --help\n");
    }
}
