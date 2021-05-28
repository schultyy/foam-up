extern crate clap;
use clap::{Arg, App};

extern crate serde;
extern crate serde_json;

mod vscode_templates;
mod project;
mod todo_file;
mod inbox_file;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("foam-up")
                          .version(VERSION)
                          .author("Jan Schulte<janschulte@fastmail.com>")
                          .about("Generates a new foam project")
                          .arg(Arg::with_name("path")
                               .short("p")
                               .long("path")
                               .value_name("PATH")
                               .help("Sets a custom path")
                               .takes_value(true)
                               .required(true))
                          .get_matches();
    println!("Developing tools for thought.");
    let path = matches.value_of("path").expect("Expected a Path");
    println!("Generating project at {}", path);
    let project = project::Project::new(path);
    project.create_directories().expect("FATAL: Could not create project directory");
    project.create_files().expect("FATAL: Could not create project files");
    println!("[x] Project created.");
}