/// Constants used to access and build the Mining Pool Servers Data Directory.
pub const STRATUMV2_FOLDER_PATH: &str = "/stratumv2/";
const STRATUMV2_KEY_FOLDER: &str = "key/";
const PRIV_STATIC_KEY: &str = "static-key.priv";
const PUB_STATIC_KEY: &str = "static-key.pub";

/// Get the path of the key folder in the Data Directory.
pub fn key_folder_path(root_dir: &str) -> String {
    root_dir.to_string() + &STRATUMV2_KEY_FOLDER.to_string()
}

/// Get the file path of the static private key in the Data Directory.
pub fn priv_key_file_path(root_dir: &str) -> String {
    root_dir.to_string() + &STRATUMV2_KEY_FOLDER.to_string() + &PRIV_STATIC_KEY.to_string()
}

/// Get the file path of the static public key in the Data Directory.
pub fn pub_key_file_path(root_dir: &str) -> String {
    root_dir.to_string() + &STRATUMV2_KEY_FOLDER.to_string() + &PUB_STATIC_KEY.to_string()
}
