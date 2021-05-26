extern crate clap;
use clap::{Arg, App};

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
}