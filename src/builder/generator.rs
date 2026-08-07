use std::{fs, io, path::Path};

const MAKEFILE_C: &str = include_str!("../../templates/Makefile_C");
const MAKEFILE_CPP: &str = include_str!("../../templates/Makefile_Cpp");

pub fn generate_makefile(
    path: &Path,
    project_name: &str,
    selected_language: &str,
) -> io::Result<()> {
    let template = match selected_language {
        "C" => MAKEFILE_C,
        "C++" => MAKEFILE_CPP,
        _ => unreachable!("Unsupported language"),
    };

    let content = template.replace("project_name", project_name);

    let makefile_path = path.join("Makefile");
    fs::write(makefile_path, content)?;

    Ok(())
}
