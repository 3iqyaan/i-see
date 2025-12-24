use clap::{Parser};

#[derive(Parser, Debug, Clone)]
#[command(name = "i-see", version = "1.0", author = "Muhammad", about = "Count the numbr of lines in a file (or some other byte)")]
pub struct Isee{
    /// Give relative or absolute path to the files you want to check in, seperated by spaces
    #[arg(short, long)]
    pub file: Vec<String>,
    /// The byte you want to search in those files
    #[arg(short, long)]
    pub byte: Option<u8>,
}