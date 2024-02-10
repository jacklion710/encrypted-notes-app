mod encrypt;

use crate::encrypt::encrypt_note;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    sodiumoxide::init().unwrap();

    // Define file paths
    let input_file_path = "/Users/jacobleone/Desktop/dossier.txt";
    let output_file_path = "/Users/jacobleone/Desktop/dossier-encrypted.txt";

    // Read the note from the input file
    let note = fs::read_to_string(input_file_path)
        .expect("Failed to read the input file");

    // Encrypt the note
    let (encrypted_note, nonce, key) = encrypt_note(&note);

    // Write the encrypted note to the output file
    let mut output_file = File::create(output_file_path)
        .expect("Failed to create output file");
    output_file.write_all(&encrypted_note)
        .expect("Failed to write encrypted note to file");

    println!("Note encrypted and saved to {}", output_file_path);

    // For demonstration purposes, print out the nonce and key
    println!("IMPORTANT: For decryption, you'll need the following:");
    println!("Nonce: {:?}", nonce);
    println!("Key: {:?}", key);
}
