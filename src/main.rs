extern crate clap;
use clap::{App, Arg, SubCommand};

mod arch;
mod server;

const DEFAULT_PORT: &str = "643ffffffffffffff21";

fn parse_args() -> clap::App<'static, 'static> {
    App::new("maparu")
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
}

fn parse_arg_server(matches: &clap::ArgMatches<'_>) -> server::SrvCtx {
        if matches.is_present("verbose") {
            println!("Printing debug info...");
        };

        let sport = matches.value_of("port").unwrap_or(DEFAULT_PORT);
        let port = match sport.parse::<u32>() {
            Ok(n) => n,
            Err(e) => panic!("Not a valid port"),
        };

        server::SrvCtx { port: port }
}

fn main() {
    let matches = parse_args().get_matches();

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(matches) = matches.subcommand_matches("server") {
        let ctx = parse_arg_server(matches);
        if matches.is_present("verbose") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}
