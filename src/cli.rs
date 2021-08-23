// cli: Handle command line parsing
use clap::{
    crate_description,
    crate_name,
    crate_version,
    App,
    Arg,
    ArgMatches,
    SubCommand,
};

// Create the App (parser)
fn create_app<'a, 'b>() -> App<'a, 'b> {
    let decrypt = SubCommand::with_name("decrypt")
        .about("Decrypt an Axiom Verge Steam file")
        .arg(
            Arg::with_name("INPUT")
                .help("File to decrypt")
                .index(1)
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("File to output to")
                .index(2)
                .takes_value(true)
        );

    let encrypt = SubCommand::with_name("encrypt")
        .about("Encrypt a file for Axiom Verge on Steam")
        .arg(
            Arg::with_name("INPUT")
                .help("File to encrypt")
                .index(1)
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("File to output encrypted content to")
                .index(2)
                .required(true)
                .takes_value(true)
        );

    let hacker = SubCommand::with_name("hacker")
        .about("Lists creatures that need glitching for the Hacker achievement")
        .arg(
            Arg::with_name("UNENCRYPTED")
                .help("Specify if operating on an unencrypted save game")
                .long("unencrypted")
                .short("u")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Save game to analyse")
                .index(1)
                .required(true)
                .takes_value(true)
        );

    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(decrypt)
        .subcommand(encrypt)
        .subcommand(hacker)
}

pub fn parse_args<'a>() -> ArgMatches<'a> {
    create_app().get_matches()
}
