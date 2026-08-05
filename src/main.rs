use clearscreen::clear;
use regex::Regex;

fn main() {
    clear().expect("Fail to clear screen");
    let c_project_name_pattern =
        Regex::new(r"^[a-z_][a-z0-9_-]*$").expect("Invalid regex pattern for C project name");

    let t1 = "teste";
    let t2 = "1teste";

    if !c_project_name_pattern.is_match(t2) {
        clear().expect("Fail to clear screen");
        println!("Failure! Invalid or unconventional name");
        //continue;
    }
}
