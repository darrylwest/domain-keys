///
/// Base62 encode and decode
/// 
/// 

use clap::Parser;
use domain_keys::base62::Base62;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, value_parser, value_name = "u64_number")]
    pub encode: Option<u64>,

    #[clap(short, long, value_parser, value_name = "base62_string")]
    pub decode: Option<String>,

    #[clap(short, long, value_parser)]
    pub quiet: bool,
}

impl Config {
    pub fn new() -> Self {
        let config = Config::parse();

        config
    }
}

fn main() {
    let config = Config::new();

    if let Some(n) = config.encode {
        println!("{}", Base62::encode(n));
    } else if let Some(s) = config.decode {
        match Base62::decode(&s) {
            Ok(n) => println!("{}", n),
            Err(err) => eprintln!("Error parsing Base62 string: {:?}", err),
        }
    } else {
        println!("\nError: must add switch to --encode or --decode; see --help\n");
    }

}