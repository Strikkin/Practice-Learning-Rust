use clap::Parser;
use args::TodoArgs;

mod args;

fn main() {
    let args = TodoArgs::parse();

    println!("{:?}", args)
}
