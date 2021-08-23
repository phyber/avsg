// cli: Handle command line parsing
use clap::{
    crate_description,
    crate_name,
    crate_version,
    App,
    Arg,
    ArgMatches,
};

// Create the App (parser)
fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("FILENAME")
                .help("Filename to operate on")
                .index(1)
                .required(true)
                .takes_value(true)
        )
}

pub fn parse_args<'a>() -> ArgMatches<'a> {
    create_app().get_matches()
}
