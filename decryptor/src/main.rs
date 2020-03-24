// Common crates
// ---------------

use std::io::Read;
use std::path::Path;
use std::fs::{read,write,OpenOptions};

// External crates
// -----------------

use dirs::desktop_dir;
use walkdir::WalkDir;
use aesstream::AesReader;
use crypto::aessafe::AesSafe256Decryptor;

// Methods
// --------

fn fetch_files
(origin: &str) -> ()
{
    // Find the Desktop
    if let Some(mut path) = desktop_dir() {
        // Find files
        let walk = WalkDir::new(origin)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
        // Retrieve the key
        path.push("golden.key");
        if let Ok(key) = read(path) {
            // Initialize the decryptor
            let decryptor: AesSafe256Decryptor = AesSafe256Decryptor::new(&key);
            // Decrypt files with the given key
            for file in walk {
                decrypt_file(file.path(), decryptor);
            }
        }
    }
}

fn decrypt_file
(path: &Path, decryptor: AesSafe256Decryptor) -> ()
{
    // Open the file
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(path) {
        // Start the reader
        if let Ok(mut reader) = AesReader::new(file,decryptor) {
            // Decrypt the content
            let mut content: Vec<u8> = Vec::<u8>::new();
            let _ = reader.read_to_end(&mut content);
            // Write it in the file
            let _ = write(path, content);
        }
    }
}

// Entrypoint
// -----------

fn main
() -> ()
{
    // Decrypt all files in
    // the current directory
    fetch_files(".");
}
