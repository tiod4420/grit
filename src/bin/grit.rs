use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
/// The stupid content tracker
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add,
    CatFile,
    CheckIgnore,
    Checkout,
    Commit,
    HashObject,
    Init,
    Log,
    LsFiles,
    LsTree,
    RevParse,
    Rm,
    ShowRef,
    Status,
    Tag,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Add => todo!(),
        Command::CatFile => todo!(),
        Command::CheckIgnore => todo!(),
        Command::Checkout => todo!(),
        Command::Commit => todo!(),
        Command::HashObject => todo!(),
        Command::Init => todo!(),
        Command::Log => todo!(),
        Command::LsFiles => todo!(),
        Command::LsTree => todo!(),
        Command::RevParse => todo!(),
        Command::Rm => todo!(),
        Command::ShowRef => todo!(),
        Command::Status => todo!(),
        Command::Tag => todo!(),
    }

    Ok(())
}
