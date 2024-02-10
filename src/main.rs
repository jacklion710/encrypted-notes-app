mod encrypt;
mod decrypt;

use crate::encrypt::{encrypt_note, save_keys};
use crate::decrypt::{decrypt_note, read_keys};
use std::env;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    sodiumoxide::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).expect("No mode specified. Use 'encrypt' or 'decrypt'.");

    match mode.as_str() {
        "encrypt" => encrypt_mode(),
        "decrypt" => decrypt_mode(&args),
        _ => println!("Invalid mode. Use 'encrypt' or 'decrypt'."),
    }
}

fn encrypt_mode() {
    // Define file paths
    let input_file_path = "/Users/juanhurtado/Downloads/build-a-palindrome-checker.txt";  // Update this path
    let output_file_path = "/Users/juanhurtado/Desktop/encrypted-build-a-palindrome-checker.txt";  // Update this path

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

    // Save the nonce and key
    save_keys(&nonce, &key, input_file_path);

    println!("Note encrypted and saved to {}", output_file_path);
}

fn decrypt_mode(args: &[String]) {
    // Ensure there are enough arguments
    if args.len() < 4 {
        println!("Usage: decrypt <encrypted_file> <output_file> <keys_file>");
        return;
    }

    let encrypted_file_path = &args[2];
    let output_file_path = &args[3];
    let keys_file_path = &args[4];

    // Read the encrypted note
    let encrypted_note = fs::read(encrypted_file_path)
        .expect("Failed to read the encrypted file");

    // Read the keys
    let (nonce, key) = read_keys(keys_file_path)
        .expect("Failed to read keys");

    // Decrypt the note
    let decrypted_note = decrypt_note(&encrypted_note, &nonce, &key)
        .expect("Failed to decrypt note");

    // Write the decrypted note to the output file
    let mut output_file = File::create(output_file_path)
        .expect("Failed to create output file");
    output_file.write_all(decrypted_note.as_bytes())
        .expect("Failed to write decrypted note to file");

    println!("Note decrypted and saved to {}", output_file_path);
}
