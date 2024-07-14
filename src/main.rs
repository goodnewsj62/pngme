use args::{Cli, Command};
use clap::Parser;
use commands::{decode, display_content, encode, remove};

mod chunk_type;
mod chunk;
mod png;
mod args;
mod commands;




fn main() {
    let cli =  Cli::parse();

    match cli.command {
        Command::Encode(EncodeArgs) =>{
            encode(EncodeArgs);
        },
        Command::Decode(DecodeArgs) =>{
            decode(DecodeArgs);
        },
        Command::Print { filepath } =>{
            display_content(&filepath.as_str())
        },
        Command::Remove { filepath, chunk_type } =>{
            remove(filepath.as_str(), chunk_type.as_str());
        }
        _ => println!("whatever")
    }
}


