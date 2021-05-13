use crate::error::{Error, Result};
use std::convert::TryInto;
use stratumv2_lib::{
    bitcoin::util::base58,
    noise::{StaticKeyPair, StaticPrivateKey},
};

/// Generate a new StaticKeyPair and returns private and public keys as base58
/// encoded strings.
pub fn new_base58_static_key() -> (String, String) {
    let static_keypair = StaticKeyPair::default();

    let secret_bytes = static_keypair.private_key.as_bytes();
    let pub_bytes = static_keypair.public_key.as_bytes();

    (
        base58::encode_slice(&secret_bytes),
        base58::encode_slice(&pub_bytes),
    )
}

/// Decodes a base58 encoded private static key and returns a StaticKeyPair.
pub fn decode_base58_static_key(base58_priv_key: &String) -> Result<StaticKeyPair> {
    let decoded = base58::from(&base58_priv_key)?;

    if decoded.len() != 32 {
        return Err(Error::ParseError(
            "Private Key length was not 32 bytes".into(),
        ));
    }

    let secret: &[u8] = &decoded[0..32];

    let priv_key = StaticPrivateKey::from_bytes(secret.try_into().unwrap());
    StaticKeyPair::from_private_key(priv_key).map_err(|e| Error::ParseError(e.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_priv_key_length() {
        let input = "foo".to_string();
        let result = decode_base58_static_key(&input);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_priv_key() {
        let input = base58::encode_slice(&[0u8; 32]);
        let result = decode_base58_static_key(&input);

        assert!(result.is_err());
    }
}
