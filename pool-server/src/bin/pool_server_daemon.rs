use clap::clap_app;
use pool_server::{constants::STRATUMV2_FOLDER_PATH, datadir::DataDir, PoolServer};
use stratumv2_lib::noise::{generate_authority_keypair, StaticKeyPair};

fn main() {
    let matches = clap_app!(PoolServerDaemon =>
      (version: "1.0")
      (author: "ccdle12")
      (about: "A CLI to start a Stratum-v2 Pool Server Daemon")
      (@arg dev: --dev "Run the server in development mode")
    )
    .get_matches();

    // Devmode will auto generate an authority keypair and static_keypair for
    // the pool server without writing to a file.
    if matches.is_present("dev") {
        let authority_keypair = generate_authority_keypair();
        let static_keypair = StaticKeyPair::default();

<<<<<<< Updated upstream
        PoolServer::new(Some(&authority_keypair), &static_keypair);
=======
        PoolServer::new(Some(&authority_keypair), &static_keypair, "127.0.0.1:8545");
>>>>>>> Stashed changes
        return;
    }

    // Creates the Data Directory if it doesn't already exist for the Mining Pool
    // Server and extracts the generated StaticKeyPair.
    let data_dir = DataDir::new(STRATUMV2_FOLDER_PATH).unwrap();
    let static_keypair = data_dir.decode_static_key().unwrap();

<<<<<<< Updated upstream
    PoolServer::new(None, &static_keypair);
=======
    PoolServer::new(None, &static_keypair, "127.0.0.1:8545");
>>>>>>> Stashed changes
}

#[cfg(test)]
mod test {
    use super::*;
    use pool_server::constants::{
        key_folder_path, priv_key_file_path, pub_key_file_path, STRATUMV2_FOLDER_PATH,
    };
    use std::fs::remove_dir_all;
    use std::path::Path;

    fn setup_tmp_dir() -> &'static str {
        let tmp_dir = "/tmp/stratumv2/";

        remove_dir_all(tmp_dir);
        DataDir::new(tmp_dir);

        tmp_dir
    }

    #[test]
    fn init_datadir() {
        let tmp_dir = setup_tmp_dir();

        // Assert the generated configuration file exists.
        assert!(Path::new(&tmp_dir).exists());
        assert!(Path::new(&key_folder_path(&tmp_dir)).exists());

        // Assert the generated static key files were generated.
        assert!(Path::new(&priv_key_file_path(&tmp_dir)).exists());
        assert!(Path::new(&pub_key_file_path(&tmp_dir)).exists());
    }
}
