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
