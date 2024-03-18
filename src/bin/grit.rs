use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use grit::repo::GitRepository;

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
    /// Initialize a new, empty repository.
    Init {
        /// Where to create the repository.
        #[arg(name = "directory", default_value = ".")]
        path: PathBuf,
    },
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
        Command::Init { path } => {
            GitRepository::create(path)?;
        }
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
