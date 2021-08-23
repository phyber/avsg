use anyhow::Result;
use serde_xml_rs::from_reader;
use std::env;
use std::process::exit;
use std::str;

mod crypto;
mod savedata;

use crypto::decrypt_file;
use savedata::THSaveData;

fn main() -> Result<()> {
    // Get a filename from the CLI
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    }
    else {
        eprintln!("Provide a file to operate on");
        exit(1)
    };

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
