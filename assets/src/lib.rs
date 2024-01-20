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
use packer_common::{fs, p_println};
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub mod buildpack;
pub use buildpack::Buildpack;

pub mod meta;
pub use meta::Meta;

pub mod builder;
pub use builder::Builder;

pub trait Assets {
    fn init_project(root: &Path, project_name: &str, config_path: &Option<PathBuf>) -> Result<()>;
}

fn new_template_engine(dir: &str) -> Tera {
    match Tera::new(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), dir)) {
        Ok(t) => t,
        Err(e) => {
            p_println!(true => "Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}

fn init_project(
    template_engine: &Tera,
    root: &Path,
    project_name: &str,
    config_path: &Option<PathBuf>,
) -> Result<()> {
    let context = init_context(project_name, config_path)?;
    write_files(template_engine, &context, root, project_name)
}

fn init_context(project_name: &str, config_path: &Option<PathBuf>) -> Result<Context> {
    let mut context = Context::new();
    context.insert("packer_name", &project_name);

    if let Some(config_path) = config_path {
        let config_json = read_config(config_path)?;
        let config_context = Context::from_serialize(config_json)?;
        context.extend(config_context);
    } else {
        // support some default value
        context.insert("dependencies", "");
    }
    Ok(context)
}

fn read_config(config_path: &Path) -> Result<serde_json::Value> {
    let config_content = fs::read_to_string(config_path)?;
    let config_value: toml::Value = toml::from_str(&config_content)?;
    let content_json = serde_json::to_value(&config_value)?;
    Ok(content_json)
}

fn write_files(
    template_engine: &Tera,
    context: &Context,
    root: &Path,
    project_name: &str,
) -> Result<()> {
    let template_names = template_engine.get_template_names();
    for template in template_names {
        let target_path = match template.starts_with("packer/") {
            true => root.join(template.replace("packer", project_name)),
            false => root.join(template),
        };
        fs::create_dir_all(target_path.parent().unwrap())?;

        let rendered = template_engine.render(template, context)?;
        fs::write(target_path, rendered)?;
    }
    Ok(())
}
