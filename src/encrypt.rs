// Add at the top of encrypt.rs
use std::{fs::File, io::Write, path::PathBuf};
use sodiumoxide::crypto::secretbox;

// Modified save_keys function in encrypt.rs to save keys in hex format
pub fn save_keys(nonce: &secretbox::Nonce, key: &secretbox::Key, original_file_path: &str) {
    let path = PathBuf::from(original_file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let keys_file_path = format!("./keys/{}-keys.txt", file_name); // Adjust path as needed

    let mut keys_file = File::create(keys_file_path).expect("Failed to create keys file");
    // Convert nonce and key to hex strings
    let nonce_hex = hex::encode(nonce);
    let key_hex = hex::encode(key);

    // Write the hex strings to the file
    keys_file.write_all(format!("Nonce: {}\nKey: {}\n", nonce_hex, key_hex).as_bytes())
        .expect("Failed to write keys to file");
}

pub fn encrypt_note(note: &str) -> (Vec<u8>, secretbox::Nonce, secretbox::Key) {
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let encrypted_note = secretbox::seal(note.as_bytes(), &nonce, &key);

    (encrypted_note, nonce, key)
}
