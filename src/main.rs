use aes::Aes128;
use anyhow::Result;
use block_modes::block_padding::Pkcs7;
use block_modes::{
    BlockMode,
    Cbc,
};
use serde_xml_rs::from_reader;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::process::exit;
use std::str;

mod savedata;
use savedata::THSaveData;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// Data encryption key
const SAVEGAME_KEY: &[u8] = &[
    186, 173, 240, 13,
    0,   0,   0,   0,
    32,  48,  68,  194,
    19,  228, 31,  u8::MAX,
];

// Encryption IV
const SAVEGAME_IV: &[u8] = &[
    229,     u8::MAX, u8::MAX, u8::MAX,
    229,     186,     7,       0,
    186,     173,     240,     13,
    u8::MAX, 0,       u8::MAX, 0,
];

// Decrypt the save data
fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    let cipher    = Aes128Cbc::new_from_slices(SAVEGAME_KEY, SAVEGAME_IV)?;
    let mut data  = data.to_owned();
    let decrypted = cipher.decrypt(&mut data)?.to_vec();

    Ok(decrypted)
}

// Read a file
fn read(filename: &str) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}

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

    let data = read(filename)?;
    let data = decrypt(&data)?;
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
