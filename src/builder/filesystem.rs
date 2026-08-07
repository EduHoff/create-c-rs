use std::{fs, io, path::Path};

pub fn create_project_structure(project_path: &Path) -> io::Result<()> {
    let src_dir = project_path.join("src");
    let include_dir = project_path.join("include");
    let build_dir = project_path.join("build");

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&include_dir)?;
    fs::create_dir_all(&build_dir)?;

    Ok(())
}
