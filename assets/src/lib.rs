extern crate self as packer_assets;

use eyre::Result;
use packer_common::Git;
use std::path::{Path, PathBuf};

pub mod buildpack;
pub use buildpack::Buildpack;

pub trait Assets {
    fn init_project(root: &Path, project_name: &str) -> Result<()>;
    fn init_git_repo(git: Git<'_>) -> Result<()>;
    fn init_workflow(root: &Path, project_name: &str) -> Result<()>;

    fn get_template_paths(
        root: &Path,
        project_name: &str,
        src: &Path,
        scripts: &Path,
        cmd: &Path,
    ) -> Vec<(PathBuf, &'static str)>;
}
