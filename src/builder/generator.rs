use std::{fs, io, path::Path};

const MAKEFILE_C: &str = include_str!("../../templates/Makefile_C");
const MAKEFILE_CPP: &str = include_str!("../../templates/Makefile_Cpp");

const MAIN_C: &str = include_str!("../../templates/main.c");
const MAIN_CPP: &str = include_str!("../../templates/main.cpp");

const README: &str = include_str!("../../templates/README.md");
const LICENSE: &str = include_str!("../../templates/LICENSE");
const GITIGNORE: &str = include_str!("../../templates/.gitignore");

const DOCKERFILE: &str = include_str!("../../templates/Dockerfile");
const DOCKER_COMPOSE: &str = include_str!("../../templates/docker-compose.yml");
const DOCKERIGNORE: &str = include_str!("../../templates/.dockerignore");

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

pub fn generate_license_file(project_path: &Path) -> io::Result<()> {
    let license_file_path = project_path.join("LICENSE");

    fs::write(license_file_path, LICENSE)?;

    Ok(())
}

pub fn generate_gitignore_file(project_path: &Path) -> io::Result<()> {
    let gitignore_file_path = project_path.join(".gitignore");

    fs::write(gitignore_file_path, GITIGNORE)?;

    Ok(())
}

pub fn generate_docker_files(project_path: &Path, project_name: &str) -> io::Result<()> {
    fs::write(project_path.join(".dockerignore"), DOCKERIGNORE)?;

    fs::write(project_path.join("Dockerfile"), DOCKERFILE)?;

    let compose_content = DOCKER_COMPOSE.replace("project_name", project_name);
    fs::write(project_path.join("docker-compose.yml"), compose_content)?;

    Ok(())
}
