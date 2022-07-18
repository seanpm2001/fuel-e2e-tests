use crate::sway::SwayProject;
use itertools::chain;
use std::error::Error;
use utils::{
    compile_sway_projects, discover_all_files_related_to_projects, env_path, track_file_changes,
};

mod sway;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = env_path("CARGO_MANIFEST_DIR")?;

    let projects = SwayProject::discover_projects(&root_dir.join("tests")).await?;

    let project_files = discover_all_files_related_to_projects(&projects).await?;
    let build_script_files = utils::all_rust_files_in(&root_dir.join("build")).await?;

    for file in chain!(&project_files, &build_script_files) {
        track_file_changes(file);
    }

    let out_dir = env_path("OUT_DIR")?;
    compile_sway_projects(projects, &out_dir.join("compiled_sway_projects")).await?;

    Ok(())
}
