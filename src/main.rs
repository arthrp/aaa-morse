mod encoder;

use clap::Parser;
use encoder::*;


#[derive(Parser, Debug)]
#[command(author, version, about = "Encode or decode a string")]
struct Args {
    /// Encode the input string
    #[arg(long, conflicts_with = "decode")]
    encode: bool,

    /// Decode the input string
    #[arg(long, conflicts_with = "encode")]
    decode: bool,

    /// The string to process
    input: String,
}

fn main() {
    let args = Args::parse();

    if args.encode {
        let res = encode(&args.input);
        println!("{}", res);
    }
    else {
        let res = decode(&args.input);
        println!("{}", res);
    }
}
