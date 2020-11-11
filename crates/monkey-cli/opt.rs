use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
pub struct Opt {
    pub file_path: Option<PathBuf>
}
