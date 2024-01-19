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

use crate::cmd::init::InitArgs;
use clap::{Parser, Subcommand};

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

/// Packer generates buildpack with just what you need to start quickly!
#[derive(Parser)]
#[clap(
    name = "packer",
    version = VERSION_MESSAGE,
    after_help = "Find more information in the url: https://github.com/amp-buildpacks/packer",
    next_display_order = None,
)]
pub struct Options {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Initialize a new project.
    Init(InitArgs),
}
