# Encrypted Notes App

This Rust project provides a simple yet powerful tool for encrypting and decrypting files using the sodiumoxide library for cryptographic operations. It's designed to be easy to use for securing your notes or any text files with strong encryption.

## Getting Started

### Prerequisites

Before running this project, ensure you have Rust and Cargo installed on your machine. If not, visit [Rust's installation](https://www.rust-lang.org/tools/install) page for detailed instructions.

## Building the Project

To build the project, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/jacklion710/encrypted-notes-app.git
cd encrypted-notes-app
```

Then, compile the project for development using Cargo:

```bash
cargo build 
```

or to build for production:

```bash
cargo build --release
```

## Usage

The app supports two modes of operation:

- encrypt: Encrypts a file and saves the encrypted version along with the keys needed for decryption.
- decrypt: Decrypts a file using the provided keys file.
- Encrypting a File
- To encrypt a file, use the encrypt mode followed by the paths for the input and output files. The app will generate a keys file in the `./keys/` directory with the same base name as the input file, appended with -keys.txt.

### Encryption flags
```bash
./encrypted-notes-app encrypt <input_file_path> <output_file_path>
```

Example:

```bash
./encrypted-notes-app encrypt /path/to/your/note.txt /path/to/encrypted/note.txt
```

### Decryption flags

To decrypt a file, use the decrypt mode followed by the paths for the encrypted file, the output file, and the keys file.

```bash
./encrypted-notes-app decrypt <encrypted_file_path> <output_file_path> <keys_file_path>
```

Example:

```bash
./encrypted-notes-app decrypt /path/to/encrypted/note.txt /path/to/decrypted/note.txt /path/to/keys/note-keys.txt
```

## Contributing

Contributions to enhance the functionality, improve the encryption methods, or fix bugs are welcome. Please feel free to fork the repository, make changes, and submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

This README provides a clear overview of your project and instructions for potential users. You can adjust paths, links, and descriptions as needed to fit your project's specifics.




