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

use lazy_static::lazy_static;
use packer_common::{fs, p_println};
use tera::{Context, Tera};

use crate::Assets;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/buildpack/**/*"
        )) {
            Ok(t) => t,
            Err(e) => {
                p_println!(true => "Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

pub struct Buildpack;

impl Assets for Buildpack {
    fn init_project(root: &std::path::Path, project_name: &str) -> eyre::Result<()> {
        let mut context = Context::new();
        context.insert("packer_name", &project_name);

        let template_names = TEMPLATES.get_template_names();
        for template in template_names {
            let target_path = match template.starts_with("packer/") {
                true => root.join(template.replace("packer", project_name)),
                false => root.join(template),
            };
            fs::create_dir_all(target_path.parent().unwrap())?;
            fs::write(target_path, TEMPLATES.render(template, &context)?)?;
        }

        Ok(())
    }
}
