use clap::{self, Parser};
use std::path::PathBuf;

const DFLAG_HELP: &str = "
Specifies a maximum depth of recursive traversing that you want to perform. The default value is infinite, meaning that the traversing is not going to stop until there are no directories found in the opened directory.

E.g for -D 4 if launched from foo/, the traversing will go no deeper than foo/bar/baz/boo and stop, dismissing all directories inside of boo.
";

const SFLAG_HELP: &str = "
Specifies whether or not to pretty-print the result. The default value (when the flag is not provided) is TRUE. 

If flag is present, the results for each file type are going to be printed each on a different line formatted like [EXTENSION] files: 9999
";

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
        help = DFLAG_HELP
    )]
    max_depth: Option<usize>,
    #[arg(long, value_name = "BOOL", help = SFLAG_HELP, short = 'n')]
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
