mod encrypt;
mod decrypt;

use std::fs::{self, File};
use std::io::Write;
use warp::Filter;
use std::collections::HashMap;
use warp::{http::StatusCode, Reply};
use serde_json::json;
use sodiumoxide::crypto::secretbox::{self, Key, Nonce};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    sodiumoxide::init().unwrap();

    // Serve files from the current directory under the `/` path.
    let static_files = warp::fs::dir("web");
    // POST endpoint for file encryption
    let encrypt_file = warp::post()
        .and(warp::path("encrypt"))
        .and(warp::body::bytes())
        .and_then(handle_encrypt);

    // POST endpoint for file decryption
    let decrypt_file = warp::post()
        .and(warp::path("decrypt"))
        .and(warp::body::bytes())
        .and_then(handle_decrypt);

    let routes = static_files.or(encrypt_file).or(decrypt_file);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn encrypt_mode(input_file_path: &str, output_file_path: &str) {
    let note = std::fs::read_to_string(input_file_path).expect("Failed to read the input file");
    let (encrypted_note, nonce, key) = crate::encrypt::encrypt_note(&note);
    let mut output_file = File::create(output_file_path).expect("Failed to create output file");
    output_file.write_all(&encrypted_note).expect("Failed to write encrypted note to file");
    crate::encrypt::save_keys(&nonce, &key, input_file_path);
    println!("Note encrypted and saved to {}", output_file_path);
}

fn decrypt_mode(encrypted_file_path: &str, output_file_path: &str, keys_file_path: &str) {
    let encrypted_note = std::fs::read(encrypted_file_path).expect("Failed to read the encrypted file");
    let (nonce, key) = crate::decrypt::read_keys(keys_file_path).expect("Failed to read keys");
    let decrypted_note = crate::decrypt::decrypt_note(&encrypted_note, &nonce, &key).expect("Failed to decrypt note");
    let mut output_file = File::create(output_file_path).expect("Failed to create output file");
    output_file.write_all(decrypted_note.as_bytes()).expect("Failed to write decrypted note to file");
    println!("Note decrypted and saved to {}", output_file_path);
}

async fn handle_encrypt(body: Bytes) -> Result<impl Reply, warp::Rejection> {
    // Attempt to convert the incoming Bytes to a string
    let body_str = match std::str::from_utf8(&body) {
        Ok(v) => v,
        Err(_) => return Err(reject::custom("Invalid request body")),
    };

    // Encrypt the note
    let encryption_result = match encrypt_note(body_str) {
        Ok((encrypted_data, nonce, key)) => {
            // Successfully encrypted, prepare the JSON response
            let response_data = json!({
                "encrypted_note": hex::encode(&encrypted_data),
                "nonce": nonce,
                "key": key,
            });

            warp::reply::json(&response_data)
        },
        Err(error_message) => {
            // Encryption failed, prepare an error JSON response
            let error_response = json!({ "error": error_message });
            return Ok(warp::reply::with_status(warp::reply::json(&error_response), StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    // Return the successful encryption result
    Ok(warp::reply::with_status(encryption_result, StatusCode::OK))
}

async fn handle_decrypt(body: bytes::Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let request: HashMap<String, String> = match serde_json::from_slice(&body) {
        Ok(map) => map,
        Err(_) => return Ok(warp::reply::with_status("Invalid JSON", StatusCode::BAD_REQUEST)),
    };

    let encrypted_note = match hex::decode(request.get("encrypted_note").unwrap_or(&String::new())) {
        Ok(v) => v,
        Err(_) => return Ok(warp::reply::with_status("Invalid encrypted note", StatusCode::BAD_REQUEST)),
    };
    let nonce = match secretbox::Nonce::from_slice(&hex::decode(request.get("nonce").unwrap_or(&String::new())).expect("Invalid nonce")) {
        Some(nonce) => nonce,
        None => return Ok(warp::reply::with_status("Invalid nonce", StatusCode::BAD_REQUEST)),
    };
    let key = match secretbox::Key::from_slice(&hex::decode(request.get("key").unwrap_or(&String::new())).expect("Invalid key")) {
        Some(key) => key,
        None => return Ok(warp::reply::with_status("Invalid key", StatusCode::BAD_REQUEST)),
    };

    let decrypted_note = match crate::decrypt::decrypt_note(&encrypted_note, &nonce, &key) {
        Ok(note) => note,
        Err(_) => return Ok(warp::reply::with_status("Decryption failed", StatusCode::INTERNAL_SERVER_ERROR)),
    };

    Ok(warp::reply::json(&response_data))
}
