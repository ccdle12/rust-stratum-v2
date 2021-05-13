use clap::clap_app;

use pool_server::keys::new_base58_static_key;
use stratumv2_lib::{bitcoin::util::base58, noise::generate_authority_keypair};

fn main() {
    let matches = clap_app!(PoolServerCLI =>
      (version: "1.0")
      (author: "ccdle12")
      (about: "A CLI for the Stratum-v2 Pool Server")
      (@subcommand dev =>
            (about: "Internal dev tools, might exist in a different project or not at all")
            (version: "1.0")
            (@arg genauthkey: -a "Generate an AuthorityKeypair, the output is a concatenation of a <32-byte private key + 32-byte public key>")
            (@arg genstatickey: -s --genstatickey "Generate a StaticKeyPair")
      )
    )
    .get_matches();

    if let Some(ref m) = matches.subcommand_matches("dev") {
        if m.is_present("genauthkey") {
            let authority_keypair = generate_authority_keypair();

            let s = base58::encode_slice(&authority_keypair.to_bytes());
            println!("{}", s);
            return;
        }

        if m.is_present("genstatickey") {
            let (priv_key, pub_key) = new_base58_static_key();
            println!("private key: {}", priv_key);
            println!("public key: {}", pub_key);
            return;
        }
    }
}
