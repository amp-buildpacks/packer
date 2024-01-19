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

use packer_common::fs;

use crate::Assets;

pub struct Buildpack;

impl Assets for Buildpack {
    fn init_project(root: &std::path::Path, project_name: &str) -> eyre::Result<()> {
        // make the dirs
        let src = root.join(project_name);
        fs::create_dir_all(&src)?;

        let scripts = root.join("scripts");
        fs::create_dir_all(&scripts)?;

        let cmd = root.join("cmd/main");
        fs::create_dir_all(&cmd)?;

        let template_paths = Self::get_template_paths(root, project_name, &src, &scripts, &cmd);
        for (target_path, content) in template_paths {
            fs::write(target_path, content.replace("{packer-name}", project_name))?;
        }

        Ok(())
    }

    fn init_git_repo(git: packer_common::Git<'_>) -> eyre::Result<()> {
        // git init
        if !git.is_in_repo()? {
            git.init()?;
        }

        // .gitignore
        let gitignore = git.root.join(".gitignore");
        if !gitignore.exists() {
            let content = include_str!("template/.gitignore");
            fs::write(gitignore, content)?;
        }

        Ok(())
    }

    fn init_workflow(root: &std::path::Path, project_name: &str) -> eyre::Result<()> {
        let github = root.join(".github");
        fs::create_dir_all(&github)?;

        let workflows = github.join("workflows");
        fs::create_dir_all(&workflows)?;

        // pipeline-descriptor.yml
        let pipeline_descriptor = github.join("pipeline-descriptor.yml");
        if !pipeline_descriptor.exists() {
            let content = include_str!("template/.github/pipeline-descriptor.yml");
            fs::write(
                pipeline_descriptor,
                content.replace("{packer-name}", project_name),
            )?;
        }

        // pb-update-pipeline.yml
        let pb_update_pipeline = workflows.join("pb-update-pipeline.yml");
        if !pb_update_pipeline.exists() {
            let content = include_str!("template/.github/workflows/pb-update-pipeline.yml");
            fs::write(pb_update_pipeline, content)?;
        }

        Ok(())
    }

    fn get_template_paths(
        root: &std::path::Path,
        project_name: &str,
        src: &std::path::Path,
        scripts: &std::path::Path,
        cmd: &std::path::Path,
    ) -> Vec<(std::path::PathBuf, &'static str)> {
        vec![
            (
                scripts.join("build.sh"),
                include_str!("template/scripts/build.sh"),
            ),
            (
                cmd.join("main.go"),
                include_str!("template/cmd/main/main.go"),
            ),
            (
                src.join("build.go"),
                include_str!("template/packer/build.go"),
            ),
            (
                src.join("detect.go"),
                include_str!("template/packer/detect.go"),
            ),
            (
                src.join(format!("{}.go", project_name)),
                include_str!("template/packer/packer.go"),
            ),
            (root.join("go.mod"), include_str!("template/go.mod")),
            (root.join("go.sum"), include_str!("template/go.sum")),
            (root.join("README.md"), include_str!("template/README.md")),
            (root.join("LICENSE"), include_str!("template/LICENSE")),
            (
                root.join("buildpack.toml"),
                include_str!("template/buildpack.toml"),
            ),
        ]
    }
}
