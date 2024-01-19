// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
