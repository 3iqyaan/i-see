use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "i-see", version = "1.0", author = "Muhammad", about = "Count the numbr of lines in a file (or some other byte)")]
pub struct Isee{

    #[command(subcommand)]
    pub target: Target
    // /// Give relative or absolute path to the files you want to check in, seperated by spaces
    // #[arg(short, long)]
    // pub file: Vec<String>,
    // /// Give some directory to recursively check all files in it
    
    // /// Activate this flag to count the number of lines
    // #[arg(short, long)]
    // pub line: bool,
    // /// Use this flag to count some byte
    // #[arg(short, long)]
    // pub byte: Option<u8>,
    // /// Use this flag to count some literal word (only works if the file is in utf-8)
    // #[arg(short, long)]
    // pub word: Option<String>,
    // /// Activate this flag to make the output more verbose
    // #[arg(short, long)]
    // pub verbose: bool,
    // /// Use this flag to count some unicode character (only works if the file is in utf-8)
    // #[arg(short, long)]
    // pub char: Option<char>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Target {
    File(FCmd),
    Dir(DCmd)
}

#[derive(Args, Debug, Clone)]
pub struct DCmd {
    /// Activate this flag to analyze files recursively in subdirectories
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// Give relative or absolute path to the directory you want to check in
    #[arg(short, long)]
    pub path: String,

    /// Optional: only analyze files with this extension
    #[arg(short, long)]
    pub extension: Option<String>,

    /// Use this flag to count some byte
    #[arg(short, long)]
    pub byte: Option<u8>,

    /// Activate this flag to count the number of lines
    #[arg(short, long)]
    pub line: bool,

    /// Activate this flag to make the output more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Activate this flag to get a report.json file in the current directory
    #[arg(short, long)]
    pub report: bool,

    /// Activate this flag to analyze ignored files based on .gitignore
    #[arg(short, long)]
    pub no_gitignore: bool,

    /// Activate this flag to analyze hidden files
    #[arg(short = 'H', long)]
    pub hidden: bool,
}

#[derive(Args, Debug, Clone)]
pub struct FCmd {
    /// Give relative or absolute path to the files you want to check in, seperated by spaces
    #[arg(short, long)]
    pub file: Vec<String>,

    /// Use this flag to count some byte
    #[arg(short, long)]
    pub byte: Option<u8>,

    /// Activate this flag to count the number of lines
    #[arg(short, long)]
    pub line: bool,

    /// Activate this flag to make the output more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Activate this flag to get a report.json file in the current directory
    #[arg(short, long)]
    pub report: bool,

    /// Activate this flag to analyze ignored files based on .gitignore
    #[arg(short, long)]
    pub no_gitignore: bool,

    /// Activate this flag to analyze hidden files
    #[arg(long)]
    pub no_hidden: bool,
}