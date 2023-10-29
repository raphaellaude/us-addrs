use clap::Parser;
use us_addrs::parse;
use us_addrs::train::train_model;

// use std::path::PathBuf;

#[derive(Parser)]
enum USAddrsCli {
    Train(TrainArgs),
    Parse(ParseArgs),
}

#[derive(Parser)]
struct TrainArgs {
    #[clap(short, long)]
    export_path: String,
}

#[derive(Parser)]
struct ParseArgs {
    #[clap(short, long)]
    address: String,
}

fn main() {
    match USAddrsCli::parse() {
        USAddrsCli::Train(args) => match train_model(&args.export_path) {
            Ok(()) => println!("Trained model"),
            Err(e) => println!("Error training model: {}", e),
        },
        USAddrsCli::Parse(args) => {
            let parsed = parse(&args.address);
            println!("{:?}", parsed);
        }
    }
}
