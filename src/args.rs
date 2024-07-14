use clap::{Args, Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Command

}

#[derive(Subcommand)]
pub enum Command{
    /// encode data
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove{
        filepath: String,
        chunk_type:String
    },
    Print {
        filepath:String
    }
}

#[derive(Args)]
pub  struct EncodeArgs {
    pub filepath: String,
    pub chunk_type:  String,
    pub message:  String,
    pub output_file:  Option<String>
}
#[derive(Args)]
pub struct DecodeArgs {
    pub filepath: String,
    pub chunk_type:  String,
}

