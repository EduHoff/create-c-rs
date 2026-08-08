use std::{fs, io, path::Path};

const MAKEFILE_C: &str = include_str!("../../templates/Makefile_C");
const MAKEFILE_CPP: &str = include_str!("../../templates/Makefile_Cpp");

const MAIN_C: &str = include_str!("../../templates/main.c");
const MAIN_CPP: &str = include_str!("../../templates/main.cpp");

const README: &str = include_str!("../../templates/README.md");

pub fn generate_makefile(
    project_path: &Path,
    project_name: &str,
    selected_language: &str,
) -> io::Result<()> {
    let template = match selected_language {
        "C" => MAKEFILE_C,
        "C++" => MAKEFILE_CPP,
        _ => unreachable!("Unsupported language"),
    };

    let content = template.replace("project_name", project_name);

    let makefile_path = project_path.join("Makefile");
    fs::write(makefile_path, content)?;

    Ok(())
}

pub fn generate_main_file(project_path: &Path, selected_language: &str) -> io::Result<()> {
    let (file_name, content) = match selected_language {
        "C" => ("main.c", MAIN_C),
        "C++" => ("main.cpp", MAIN_CPP),
        _ => unreachable!("Unsupported language"),
    };

    let main_file_path = project_path.join("src").join(file_name);

    fs::write(main_file_path, content)?;

    Ok(())
}

pub fn generate_readme_file(project_path: &Path, project_name: &str) -> io::Result<()> {
    let content = README.replace("project_name", project_name);

    let readme_file_path = project_path.join("README.md");

    fs::write(readme_file_path, content)?;

    Ok(())
}
