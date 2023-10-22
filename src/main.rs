use clap::Parser;
use us_addrs::parse;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    address: String,
}

fn main() {
    let args = Args::parse();
    let address = args.address;
    let parsed = parse(&address);
    println!("{:?}", parsed);
}
