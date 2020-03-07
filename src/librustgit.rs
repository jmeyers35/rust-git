use ini::Ini;
use std::fs::{create_dir, metadata, File};
use std::io::Write;
use std::path::PathBuf;

struct Repository {
    work_tree: PathBuf,
    git_dir: PathBuf,
    conf: Ini,
}

enum GitError {
    InvalidGitRepository(String),
}

impl Repository {
    pub fn new(path: PathBuf) -> Result<Repository, GitError> {
        let mut git_path = path.clone();
        git_path.push(".git");
        if !git_path.exists() {
            return Err(GitError::InvalidGitRepository(format!(
                "{} not valid git repository: no .git directory found",
                path.to_str().unwrap()
            )));
        }
        let mut conf_path = path.clone();
        conf_path.push("config");
        let conf;
        if let Ok(ini_conf) = Ini::load_from_file(conf_path) {
            conf = ini_conf;
        } else {
            return Err(GitError::InvalidGitRepository(format!(
                "{} not valid git repository: invalid or missing configuration file",
                path.to_str().unwrap()
            )));
        }

        Ok(Repository {
            work_tree: path,
            git_dir: git_path,
            conf,
        })
    }
}

pub fn create_repository(path: PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        // create directory and new structure
        create_dir(&path)?;
    } else {
        let metadata = metadata(&path)?;
        if !metadata.is_dir() {
            panic!("Not a directory!"); // TODO acutal error handling
        }
        let is_empty = path.read_dir()?.next().is_none();
        if !is_empty {
            panic!("Directory not empty!"); // TODO actual error handling
        }
    }
    create_git_directory_structure(path)
}

fn create_git_directory_structure(path: PathBuf) -> std::io::Result<()> {
    // worktree/.git
    let mut git_path = path.clone();
    git_path.push(".git");
    create_dir(&git_path)?;

    // .git/objects
    git_path.push("objects");
    create_dir(&git_path)?;

    // .git/branches
    git_path.set_file_name("branches");
    create_dir(&git_path)?;

    // .git/refs/heads
    git_path.set_file_name("refs");
    create_dir(&git_path)?;
    git_path.push("heads");
    File::create(&git_path)?;
    // .git/refs/tags
    git_path.set_file_name("tags");
    File::create(&git_path)?;

    // .git/description
    git_path.pop();
    git_path.set_file_name("description");
    let mut desc_handle = File::create(&git_path)?;
    let desc_buf =
        "Unnamed repository; edit this file 'description' to name the repository.\n".as_bytes();
    File::write_all(&mut desc_handle, desc_buf)?;

    // .git/HEAD
    git_path.set_file_name("HEAD");
    let mut head_handle = File::create(&git_path)?;
    let head_buf = "ref: refs/heads/master\n".as_bytes();
    File::write_all(&mut head_handle, head_buf)?;

    // .git/config
    git_path.set_file_name("config");
    let default_config = default_repo_config();
    default_config.write_to_file(&git_path)?;
    Ok(())
}

fn default_repo_config() -> Ini {
    let mut ini = Ini::new();
    ini.with_section(Some("core"))
        .set("repositoryformatversion", "0")
        .set("filemode", "false")
        .set("bare", "false");

    ini
}
