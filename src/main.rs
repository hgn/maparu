extern crate clap;
use clap::{App, Arg, SubCommand};

mod arch;
mod server;

const DEFAULT_PORT: &str = "64321";

fn main() {
    let matches = App::new("maparu")
        .version("1.0.0")
        .author("Hagen Paul Pfeifer <hagen@jauu.net>")
        .about("Maparo Server and Client")
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("starts maparu server instance")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .help("print information verbosely, may limit performance"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .short("port")
                        .takes_value(true)
                        .help("port for control"),
                ),
        )
        .subcommand(
            SubCommand::with_name("client")
                .about("maparu measurement client")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .help("print information verbosely, may limit performance"),
                ),
        )
        .get_matches();


	match matches.occurrences_of("v") {
		0 => println!("No verbose info"),
		1 => println!("Some verbose info"),
		2 => println!("Tons of verbose info"),
		3 | _ => println!("Don't be crazy"),
	}

	if let Some(matches) = matches.subcommand_matches("server") {
        let port = matches.value_of("port").unwrap_or(DEFAULT_PORT);
		if matches.is_present("verbose") {
			println!("Printing debug info...");
		} else {
			println!("Printing normally...");
		}
	}
}
