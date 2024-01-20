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

use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use tera::Tera;

use crate::{init_project, new_template_engine, Assets};

lazy_static! {
    pub static ref BUILDER_TEMPLATE_ENGINE: Tera = new_template_engine("/templates/builder/**/*");
}

pub struct Builder;

impl Assets for Builder {
    fn init_project(
        root: &Path,
        project_name: &str,
        config_path: &Option<PathBuf>,
    ) -> eyre::Result<()> {
        init_project(
            BUILDER_TEMPLATE_ENGINE.deref(),
            root,
            project_name,
            config_path,
        )
    }
}
