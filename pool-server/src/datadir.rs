use crate::{
    constants::{key_folder_path, priv_key_file_path, pub_key_file_path},
    error::Result,
    keys::{decode_base58_static_key, new_base58_static_key},
};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{prelude::*, BufReader, BufWriter},
    path::Path,
};
use stratumv2_lib::noise::StaticKeyPair;

/// Represents the Data Directory for the Mining Pool Server.
pub struct DataDir<'a> {
    root_dir: &'a str,
}

impl<'a> DataDir<'a> {
    /// Initialize the Data Directory by creating the folders and generating
    /// static key files if they don't exist.
    pub fn new(root_dir: &'a str) -> Result<DataDir> {
        Self::create_datadir(root_dir)?;
        Self::generate_static_key_files(root_dir)?;

        Ok(DataDir { root_dir })
    }

    /// Create all the required folders for the Data Directory.
    fn create_datadir(root_dir: &str) -> Result<()> {
        let buffer = [root_dir.to_string(), key_folder_path(&root_dir)];

        for s in buffer {
            create_dir_all(s)?;
        }

        Ok(())
    }

    /// Generate the StaticKey files for the Mining Pool Server if they don't
    /// already exist.
    fn generate_static_key_files(root_dir: &str) -> Result<()> {
        if !Path::new(&priv_key_file_path(root_dir)).exists() {
            let (private_key, public_key) = new_base58_static_key();

            // Write the private and public static keys to separate files
            // according to the path provided.
            let buffer = [
                (priv_key_file_path(root_dir), private_key),
                (pub_key_file_path(root_dir), public_key),
            ];

            for t in buffer {
                Self::write_to_file(&t.0, &t.1)?;
            }
        }

        Ok(())
    }

    fn write_to_file(root_dir: &str, content: &str) -> Result<()> {
        let file_handler = OpenOptions::new().create(true).write(true).open(root_dir)?;

        let mut writer = BufWriter::new(file_handler);
        writer.write(content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    /// Decodes the Private Static Key found in the Data Directory and returns
    /// an in-memory struct for the StaticKeyPair.
    pub fn decode_static_key(&self) -> Result<StaticKeyPair> {
        let priv_key_path = priv_key_file_path(&self.root_dir);
        let file_handler = OpenOptions::new().read(true).open(priv_key_path)?;

        let mut reader = BufReader::new(file_handler);
        let mut static_key = String::new();
        reader.read_line(&mut static_key)?;

        decode_base58_static_key(&static_key)
    }
}
