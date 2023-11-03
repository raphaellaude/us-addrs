use clap::Parser;
use us_addrs::train::train_model;
use us_addrs::{parse, parse_addresses_from_txt, TAGS};

// use std::path::PathBuf;

#[derive(Parser)]
enum USAddrsCli {
    Train(TrainArgs),
    Parse(ParseArgs),
    ParseFile(ParseFileArgs),
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

#[derive(Parser)]
struct ParseFileArgs {
    #[clap(short, long)]
    file_path: String,
    export_path: String,
}

fn main() {
    match USAddrsCli::parse() {
        USAddrsCli::Train(args) => match train_model(&args.export_path) {
            Ok(()) => println!("Trained model"),
            Err(e) => println!("Error training model: {}", e),
        },
        USAddrsCli::ParseFile(args) => {
            let parsed_addresses = parse_addresses_from_txt(&args.file_path);
            // write as CSV with Tags as columns
            let mut wtr = csv::Writer::from_path(&args.export_path).unwrap();

            wtr.write_record(TAGS.iter()).unwrap();

            for tagged_address in parsed_addresses {
                let mut record = Vec::new();

                for tag in TAGS.iter() {
                    if let Some((token, _)) = tagged_address
                        .iter()
                        .find(|&(_, token_tag)| *token_tag == *tag)
                    {
                        record.push(token.to_string());
                    } else {
                        record.push("".to_string());
                    }
                }
                wtr.write_record(&record).unwrap();
            }
            wtr.flush().unwrap();
        }
        USAddrsCli::Parse(args) => {
            let parsed = parse(&args.address);
            println!("{:?}", parsed);
        }
    }
}
