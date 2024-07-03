use crate::args::Args;
use crate::args::Commands::{Decode, Encode, Print, Remove};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Encode {
            path,
            chunk_type,
            message,
        } => {
            print!("Encode called: {}, {}, {}", path, chunk_type, message);
        }
        Decode { path, chunk_type } => {
            print!("Decode called: {}, {}", path, chunk_type);
        }
        Remove { path, chunk_type } => {
            print!("Remove called: {}, {}", path, chunk_type);
        }
        Print { path } => {
            print!("Print called: {}", path);
        }
    }
    Ok(())
}
