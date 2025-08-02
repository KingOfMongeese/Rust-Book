#![warn(clippy::pedantic)]

mod cli;

use cli::Args;
use clap::Parser;
use minigrep::Config;
use anyhow::Result;

fn main() -> Result<()>{
    let args = Args::parse();
    let config = Config::build(args.search_string, args.file_name, args.do_print_color);

    minigrep::run(config)
}
