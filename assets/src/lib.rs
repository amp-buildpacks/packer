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
use std::path::Path;
use tera::{Context, Tera};

pub mod buildpack;
pub use buildpack::Buildpack;

pub mod meta;
pub use meta::Meta;

pub mod builder;
pub use builder::Builder;

pub trait Assets {
    fn init_project(root: &Path, project_name: &str) -> Result<()>;
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
    root: &std::path::Path,
    project_name: &str,
) -> eyre::Result<()> {
    let mut context = Context::new();
    context.insert("packer_name", &project_name);
    // TODO: issue #7 Support custom configuration file
    context.insert("dependencies", "");

    let template_names = template_engine.get_template_names();
    for template in template_names {
        let target_path = match template.starts_with("packer/") {
            true => root.join(template.replace("packer", project_name)),
            false => root.join(template),
        };
        fs::create_dir_all(target_path.parent().unwrap())?;
        fs::write(target_path, template_engine.render(template, &context)?)?;
    }

    Ok(())
}
