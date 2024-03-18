use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use ini::Ini;

use crate::Result;

const DEFAULT_BRANCH: &str = "master";
const DEFAULT_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.";

#[derive(Debug)]
pub struct GitRepository {
    gitdir: PathBuf,
    worktree: PathBuf,
    config: Ini,
}

impl GitRepository {
    pub fn create(worktree: impl AsRef<Path>) -> Result<Self> {
        let worktree = worktree.as_ref();

        let repo = Self::new(worktree, true)?;

        if repo.worktree.exists() {
            if !repo.worktree.is_dir() {
                return Err(format!("{} is not a directory", repo.worktree.display()).into());
            }

            let is_empty = repo.gitdir.read_dir().unwrap().count() == 0;
            if repo.gitdir.exists() && is_empty {
                return Err(format!("{} is not empty", repo.gitdir.display()).into());
            }
        } else {
            fs::create_dir_all(worktree)?;
        }

        // .git/branches
        let branches = repo.dir(["branches"])?;
        fs::create_dir_all(branches)?;

        // .git/objects
        let objects = repo.dir(["objects"])?;
        fs::create_dir_all(objects)?;

        // .git/refs/heads
        let heads = repo.dir(["refs", "heads"])?;
        fs::create_dir_all(heads)?;

        // .git/refs/tags
        let tags = repo.dir(["refs", "tags"])?;
        fs::create_dir_all(tags)?;

        // .git/config
        let mut config = File::create(repo.path(["config"]))?;
        Self::default_config().write_to(&mut config)?;

        // .git/description
        let mut description = File::create(repo.path(["description"]))?;
        writeln!(description, "{}", DEFAULT_DESCRIPTION)?;

        // .git/HEAD
        let mut head = File::create(repo.path(["HEAD"]))?;
        writeln!(head, "ref: refs/heads/{}", DEFAULT_BRANCH)?;

        Ok(repo)
    }

    pub fn find(path: Option<impl AsRef<Path>>, required: bool) -> Result<Option<Self>> {
        let path = match path {
            Some(path) => path.as_ref().to_owned(),
            None => PathBuf::from("."),
        };

        let path = fs::canonicalize(path)?;

        for path in path.ancestors() {
            if path.join(".git").is_dir() {
                let repo = Self::new(path, false)?;
                return Ok(Some(repo));
            }
        }

        if !required {
            Ok(None)
        } else {
            Err("No git repository.".into())
        }
    }

    fn new(worktree: impl AsRef<Path>, force: bool) -> Result<Self> {
        let worktree = worktree.as_ref().to_owned();
        let gitdir = worktree.join(".git");
        let config = gitdir.join("config");

        if !force && !gitdir.is_dir() {
            return Err(format!("Not a Git repository {}", gitdir.display()).into());
        }

        let config = match Ini::load_from_file(config) {
            Ok(config) => config,
            Err(_) if force => Ini::new(),
            Err(_) => Err("Configuration file missing")?,
        };

        let version = config
            .get_from(Some("core"), "repositoryformatversion")
            .and_then(|val| val.parse::<i32>().ok())
            .unwrap_or(-1);

        if !force && version != 0 {
            return Err(format!("Unsupported repositoryformatversion {}", version).into());
        }

        Ok(Self {
            worktree,
            gitdir,
            config,
        })
    }

    fn path<P: AsRef<Path>>(&self, paths: impl AsRef<[P]>) -> PathBuf {
        paths
            .as_ref()
            .iter()
            .map(AsRef::as_ref)
            .fold(self.gitdir.clone(), |acc, path| acc.join(path))
    }

    fn dir<P: AsRef<Path>>(&self, paths: impl AsRef<[P]>) -> Result<PathBuf> {
        let path = self.path(paths);

        if path.is_dir() {
            Ok(path)
        } else if path.exists() {
            Err(format!("Not a directory {}", path.display()).into())
        } else {
            Err(format!("Directory does not exist {}", path.display()).into())
        }
    }

    fn default_config() -> Ini {
        let mut config = Ini::new();

        config
            .with_section(Some("core"))
            .set("bare", "false")
            .set("repositoryformatversion", "0")
            .set("filemode", "false");

        config
    }
}
