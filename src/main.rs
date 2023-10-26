use clap::Parser;
use us_addrs::parse;
use us_addrs::train::train_model;

// use std::path::PathBuf;

#[derive(Parser)]
enum USAddrsCli {
    Train,
    Parse(ParseArgs),
}

#[derive(Parser)]
struct ParseArgs {
    #[clap(short, long)]
    address: String,
}

fn main() {
    match USAddrsCli::parse() {
        USAddrsCli::Train => match train_model() {
            Ok(()) => println!("Trained model"),
            Err(e) => println!("Error training model: {}", e),
        },
        USAddrsCli::Parse(args) => {
            let parsed = parse(&args.address);
            println!("{:?}", parsed);
        }
    }
}
