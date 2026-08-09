use std::path::Path;

use clearscreen::clear;
use create_c_rs::builder::{
    filesystem::create_project_structure,
    generator::{
        generate_docker_files, generate_gitignore_file, generate_license_file, generate_main_file,
        generate_makefile, generate_readme_file,
    },
};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use regex::Regex;

fn main() {
    clear().expect("Fail to clear screen");
    let c_project_name_pattern = Regex::new(r"^[a-z0-9][a-z0-9_.-]*[a-z0-9]$")
        .expect("Invalid regex pattern for C/Docker project name");

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write C/C++ project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if c_project_name_pattern.is_match(input) {
                Ok(())
            } else {
                Err("Invalid or unconventional name! Must match C/Docker requirements")
            }
        })
        .interact_text()
        .expect("Failed to get project name");

    let languages = ["C", "C++"];
    let language_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the language of the project:")
        .items(languages)
        .default(0)
        .interact()
        .expect("Failed to select language");

    let selected_language = languages
        .get(language_index)
        .expect("Selected index should always be within bounds of languages array");

    let project_path = Path::new(&project_name);

    if let Err(err) = create_project_structure(project_path) {
        eprintln!("Error creating the folder structure: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_makefile(project_path, &project_name, selected_language) {
        eprintln!("Error generating Makefile: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_main_file(project_path, selected_language) {
        eprintln!("Error generating main file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_readme_file(project_path, &project_name) {
        eprintln!("Error generating README file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_license_file(project_path) {
        eprintln!("Error generating LICENSE file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_gitignore_file(project_path) {
        eprintln!("Error generating .gitignore file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_docker_files(project_path, &project_name) {
        eprintln!("Error generating Docker files: {err}");
        std::process::exit(1);
    }

    clear().expect("Fail to clear screen");
    println!("Project name: {project_name}");
    println!("Language: {selected_language}");

    #[cfg(windows)]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("pause")
        .status();
}
