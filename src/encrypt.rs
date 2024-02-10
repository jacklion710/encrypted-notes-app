// encrypt.rs
extern crate sodiumoxide;

use sodiumoxide::crypto::secretbox;

pub fn encrypt_note(note: &str) -> (Vec<u8>, secretbox::Nonce, secretbox::Key) {
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let encrypted_note = secretbox::seal(note.as_bytes(), &nonce, &key);
    (encrypted_note, nonce, key)
}
