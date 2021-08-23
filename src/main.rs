// avsg: Simple tool to look at the Axiom Verge save file.
//
// This mostly assists with the Steam version of the game, since those save
// files are encrypted.
#![forbid(unsafe_code)]
use anyhow::Result;
use clap::ArgMatches;
use serde_xml_rs::from_reader;
use std::fs::OpenOptions;
use std::io::{
    self,
    prelude::*,
};

mod cli;
mod crypto;
mod savedata;

use crypto::{
    decrypt_file,
    encrypt_file,
};
use savedata::THSaveData;

fn decrypt(matches: &ArgMatches) -> Result<()> {
    let filename = matches.value_of("INPUT").unwrap();

    // If we got an output argument, we write to that file, otherwise write
    // to STDOUT
    let output = matches.value_of("OUTPUT");
    let mut output: Box<dyn io::Write> = if let Some(output) = output {
        // Write to a new file. Error out if the file already exists to avoid
        // accidentally overwriting things.
        let fh = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(output)?;

        Box::new(fh)
    }
    else {
        let stdout = io::stdout();

        Box::new(stdout)
    };

    // Decrypt the data and write it to the output
    let data = decrypt_file(filename)?;
    output.write_all(&data)?;

    Ok(())
}

fn encrypt(matches: &ArgMatches) -> Result<()> {
    // Both are required and safe to unwrap.
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    encrypt_file(input, output)?;

    Ok(())
}

fn hacker(matches: &ArgMatches) -> Result<()> {
    // Required, safe to unwrap
    let filename = matches.value_of("INPUT").unwrap();

    let data = if matches.is_present("UNENCRYPTED") {
        // File is unencrypted, we can just read it normally.
        let mut fh = OpenOptions::new()
            .read(true)
            .open(filename)?;

        let mut buffer = Vec::new();
        fh.read_to_end(&mut buffer)?;

        buffer
    }
    else {
        // Encrypted file, probably Steam. Decrypt it.
        decrypt_file(filename)?
    };

    let savedata: THSaveData = from_reader(&*data)?;

    //println!("{:#?}", savedata);

    if let Some(remaining) = savedata.hacker_requires() {
        let num = remaining.len();
        let word = if num > 1 {
            "creatures"
        }
        else {
            "creature"
        };

        println!("Hacker Achievement requires {} more {}:", num, word);

        for creature in remaining {
            println!("  - {} ({:?})", creature, creature);
        }
    }
    else {
        println!("Hacker Achievement requires:");
        println!("  - All creatures required");
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = cli::parse_args();

    // Act on subcommands.
    match args.subcommand() {
        // Simply decrypt the given file
        ("decrypt", Some(matches)) => {
            decrypt(matches)?
        },

        // Encrypt a file
        ("encrypt", Some(matches)) => {
            encrypt(matches)?
        },

        // View details for Hacker achievement
        ("hacker", Some(matches)) => {
            hacker(matches)?
        }

        // Unreachable
        (_, _) => unreachable!(),
    }

    Ok(())
}
