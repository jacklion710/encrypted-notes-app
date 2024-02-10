use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::io::{self, Read};
use hex::FromHex; // Ensure the `hex` crate is added to your Cargo.toml dependencies

pub fn decrypt_note(encrypted_note: &[u8], nonce: &secretbox::Nonce, key: &secretbox::Key) -> Result<String, io::Error> {
    secretbox::open(encrypted_note, nonce, key)
        .map(|decrypted| String::from_utf8(decrypted).expect("Failed to convert decrypted bytes to String"))
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Decryption failed"))
}

pub fn read_keys(file_path: &str) -> io::Result<(secretbox::Nonce, secretbox::Key)> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines = contents.lines();

    let nonce_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Nonce not found"))?;
    let key_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Key not found"))?;

    let nonce_str = nonce_line.trim_start_matches("Nonce: ").trim();
    let key_str = key_line.trim_start_matches("Key: ").trim();

    let nonce_bytes = hex::decode(nonce_str).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid nonce hex"))?;
    let key_bytes = hex::decode(key_str).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid key hex"))?;

    let nonce = secretbox::Nonce::from_slice(&nonce_bytes).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid nonce"))?;
    let key = secretbox::Key::from_slice(&key_bytes).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid key"))?;

    Ok((nonce, key))
}
