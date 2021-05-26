extern crate clap;
use clap::{Arg, App};

extern crate serde;
#[macro_use]
extern crate serde_json;

mod vscode_templates;
mod project;
mod todo_file;
mod inbox_file;

fn main() {
    let matches = App::new("foam-up")
                          .version("1.0")
                          .author("Jan Schulte<janschulte@fastmail.com>")
                          .about("Generates a new foam project")
                          .arg(Arg::with_name("path")
                               .short("p")
                               .long("path")
                               .value_name("PATH")
                               .help("Sets a custom path")
                               .takes_value(true))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let path = matches.value_of("path").unwrap_or(".");
    println!("Value for path: {}", path);

    let project = project::Project::new(path);
    project.create_directories().expect("FATAL: Could not create project directory");
    project.create_files().expect("FATAL: Could not create project files");
}