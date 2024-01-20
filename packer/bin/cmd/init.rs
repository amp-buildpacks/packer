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

use clap::{Parser, ValueHint};
use eyre::Result;
use packer_assets::{Assets, Builder, Buildpack, Meta};
use packer_common::{fs, p_println, Git};
use std::path::PathBuf;
use yansi::Paint;

/// CLI arguments for `forge init`.
#[derive(Clone, Debug, Parser)]
pub struct InitArgs {
    /// The root directory of the new project.
    #[clap(value_hint = ValueHint::DirPath, default_value = ".", value_name = "PROJECT_NAME")]
    root: PathBuf,

    /// The template to use for the new project, Support buildpack, meta and builder.
    #[clap(long, short, default_value = "buildpack")]
    template: String,

    /// Create the project even if the specified root directory is not empty.
    #[clap(long, short)]
    force: bool,

    /// Suppress all output.
    #[clap(long, short)]
    quiet: bool,
}

impl InitArgs {
    pub fn run(self) -> Result<()> {
        let InitArgs {
            root,
            template,
            force,
            quiet,
        } = self;

        // create the root dir if it does not exist
        if !root.exists() {
            fs::create_dir_all(&root)?;
        }

        let root = dunce::canonicalize(root)?;
        let git = Git::new(&root);

        // if target is not empty
        if root.read_dir().map_or(false, |mut i| i.next().is_some()) {
            if !force {
                eyre::bail!(
                    "Cannot run `init` on a non-empty directory.\n\
                    Run with the `--force` flag to initialize regardless."
                );
            }
            p_println!(!quiet => "Target directory is not empty, but `--force` was specified");
        }

        let project_name = root.file_name().unwrap().to_string_lossy().into_owned();

        p_println!(!quiet => "Initializing {}...", root.display());

        match template.as_str() {
            "buildpack" => Buildpack::init_project(&root, &project_name)?,
            "meta" => Meta::init_project(&root, &project_name)?,
            "builder" => Builder::init_project(&root, &project_name)?,
            _ => eyre::bail!("Unknown template: {}", template),
        }

        init_git_repo(git)?;

        p_println!(!quiet => "    {} packer project: {}",  Paint::green("Initialized"), Paint::yellow(project_name.as_str()));
        Ok(())
    }
}

fn init_git_repo(git: Git<'_>) -> eyre::Result<()> {
    // git init
    if !git.is_in_repo()? {
        git.init()?;
    }
    Ok(())
}
