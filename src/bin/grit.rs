use std::error::Error;
use std::io::{stdout, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use grit::object::GitObject;
use grit::repo::GitRepository;

#[derive(Parser, Debug)]
#[command(version, about)]
/// The stupidest content tracker
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add,
    /// Provide content of repository objects
    CatFile {
        /// Specify the type
        #[arg(name = "type", default_value = "blob")]
        obj_type: GitObjectType,
        /// The object to display
        object: String,
    },
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

#[derive(ValueEnum, Clone, Debug)]
pub enum GitObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Add => todo!(),
        Command::CatFile { object, .. } => {
            let repo = GitRepository::find(".")?;
            let object = GitObject::read(&repo, object)?;
            let data = object.serialize()?;
            stdout().write_all(data)?;
        }
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
