use std::error::Error;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use grit::object::{GitObject, GitObjectType};
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
    /// Provide content of repository objects
    CatFile {
        /// Specify the type
        #[arg(name = "type")]
        obj: GitObjectType,
        /// The object to display
        #[arg(name = "object")]
        sha: String,
    },
    CheckIgnore,
    Checkout,
    Commit,
    /// Compute object ID and optionally creates a blob from a file
    HashObject {
        /// Specify the type
        #[arg(short = 't', name = "type", default_value = "blob")]
        obj: GitObjectType,
        /// Actually write the object into the database
        #[arg(short)]
        write: bool,
        /// Read object from <file>
        #[arg(name = "file")]
        path: PathBuf,
    },
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
        Command::CatFile { sha, .. } => {
            let repo = GitRepository::find(".")?;
            let data = repo.read(sha)?.serialize();
            stdout().write_all(&data)?;
        }
        Command::CheckIgnore => todo!(),
        Command::Checkout => todo!(),
        Command::Commit => todo!(),
        Command::HashObject { obj, write, path } => {
            let data = fs::read(path)?;
            let object = GitObject::create(obj, data);
            if write {
                let repo = GitRepository::find(".")?;
                repo.write(&object)?
            }
            println!("{}", object.hash());
        }
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
