use aes::Aes128;
use anyhow::Result;
use block_modes::block_padding::Pkcs7;
use block_modes::{
    BlockMode,
    Cbc,
};
use std::io::prelude::*;
use std::fs::{
    File,
    OpenOptions,
};
use std::str;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// Data encryption key
const SAVEGAME_KEY: &[u8] = &[
    186, 173, 240, 13,
    0,   0,   0,   0,
    32,  48,  68,  194,
    19,  228, 31,  255,
];

// Encryption IV
const SAVEGAME_IV: &[u8] = &[
    229, 255, 255, 255,
    229, 186, 7,   0,
    186, 173, 240, 13,
    255, 0,   255, 0,
];

// Read a file
fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut f      = File::open(filename)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}

// Decrypt the save data
fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    let cipher    = Aes128Cbc::new_from_slices(SAVEGAME_KEY, SAVEGAME_IV)?;
    let mut data  = data.to_owned();
    let decrypted = cipher.decrypt_vec(&mut data)?;

    Ok(decrypted)
}

// Read and decrypt a file
pub fn decrypt_file(filename: &str) -> Result<Vec<u8>> {
    let data      = read_file(filename)?;
    let decrypted = decrypt(&data)?;

    Ok(decrypted)
}

// Encrypt a plain text file
fn encrypt(data: &[u8]) -> Result<Vec<u8>> {
    let cipher    = Aes128Cbc::new_from_slices(SAVEGAME_KEY, SAVEGAME_IV)?;
    let mut data  = data.to_owned();
    let encrypted = cipher.encrypt_vec(&mut data);

    Ok(encrypted)
}

// Encrypt a given input and write it to the given output
pub fn encrypt_file(input: &str, output: &str) -> Result<()> {
    let mut output = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)?;

    let data = read_file(input)?;
    let data = encrypt(&data)?;

    output.write_all(&data)?;

    Ok(())
}
