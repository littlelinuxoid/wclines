use clap::{self, Parser};
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(num_args = 1, value_name = "PATH", required = true)]
    file_name: PathBuf,
    #[arg(
        num_args = 1,
        short = 'D',
        long,
        value_name = "INT",
        help = "Hello World!"
    )]
    max_depth: Option<usize>,
    #[arg(long, value_name = "BOOL", help = "Hello World!", short = 'S')]
    no_pretty_print: bool,
}

pub fn parseargs() -> PathBuf {
    let arguments = Cli::parse();
    if let Some(depth) = arguments.max_depth {
        println!("Specified Maximum Recursion Depth: {depth}");
    }
    if arguments.no_pretty_print {
        println!("You decided not to PP!")
    } else {
        println!("You decided to PP!");
    }
    arguments.file_name
}
