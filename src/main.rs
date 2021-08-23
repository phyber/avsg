// avsg: Simple tool to look at the Axiom Verge save file.
//
// This mostly assists with the Steam version of the game, since those save
// files are encrypted.
#![forbid(unsafe_code)]
use anyhow::Result;
use serde_xml_rs::from_reader;
use std::str;

mod cli;
mod crypto;
mod savedata;

use crypto::decrypt_file;
use savedata::THSaveData;

fn main() -> Result<()> {
    let args = cli::parse_args();

    // Required, safe to unwrap
    let filename = args.value_of("FILENAME").unwrap();

    let data = decrypt_file(filename)?;
    //println!("DATA: {:?}", data);

    let xml = str::from_utf8(&data)?;
    println!("{}", xml);

    let savedata: THSaveData = from_reader(&*data)?;
    println!("{:#?}", savedata);


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
