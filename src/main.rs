use clap::{Parser, Subcommand};

mod precmd;
mod prompt;

#[derive(Parser, Debug)]
#[command(name = "purs", author, version, about, long_about = None, propagate_version = true)]
pub struct Purs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Precmd(precmd::Precmd),
    Prompt(prompt::Prompt),
}

fn main() {
    match Purs::parse().command {
        Command::Precmd(precmd) => precmd::display(precmd),
        Command::Prompt(prompt) => prompt::display(prompt),
    }
}
