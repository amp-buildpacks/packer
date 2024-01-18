mod cmd;
mod options;
use clap::Parser;
use options::{Options, Subcommands};

fn main() -> Result<(), eyre::Error> {
    let options = Options::parse();
    match options.sub {
        Subcommands::Init(cmd) => cmd.run(),
    }
}
