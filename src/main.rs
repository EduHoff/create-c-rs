use clearscreen::clear;
use create_c_rs::utils::input;
use regex::Regex;

fn main() {
    clear().expect("Fail to clear screen");
    let c_project_name_pattern =
        Regex::new(r"^[a-z_][a-z0-9_-]*$").expect("Invalid regex pattern for C project name");

    loop {
        let project_name = input::get_input("Write C/C++ project name: ");

        if c_project_name_pattern.is_match(&project_name) {
            println!("{project_name}");
            break;
        }
        clear().expect("Fail to clear screen");
        println!("Failure! Invalid or unconventional name");
    }
}
