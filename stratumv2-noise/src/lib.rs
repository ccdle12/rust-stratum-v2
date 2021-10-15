mod certificate_format;
mod error;
mod noise_session;
mod signature_noise_message;
mod signed_certificate;
mod types;

pub extern crate bitcoin;

pub use certificate_format::CertificateFormat;
pub use noise_session::{NoiseSession, Session};
pub use signature_noise_message::SignatureNoiseMessage;
pub use signed_certificate::SignedCertificate;
pub use types::{
    AuthKey, AuthorityKeyPair, AuthorityPublicKey, Signature, StaticKey, StaticKeyPair,
    StaticPublicKey,
};

#[cfg(test)]
mod test {
    use crate::{
        certificate_format::CertificateFormat,
        error::Error,
        noise_session::{NoiseSession, Session},
        signature_noise_message::SignatureNoiseMessage,
        signed_certificate::{authority_sign_cert, SignedCertificate},
        types::{
            AuthKey, AuthorityKeyPair, AuthorityPublicKey, StaticKey, StaticKeyPair,
            StaticPublicKey,
        },
    };
    use bitcoin::util::base58;
    use noiseexplorer_nx::types::Keypair;
    use rand::rngs::OsRng;
    use std::{
        thread::sleep,
        time::{Duration, SystemTime},
    };
    use stratumv2_serde::{
        deserialize,
        types::unix_timestamp::{system_unix_time_to_u32, unix_u32_now},
        Serializable,
    };

    // Helper function to generate timestamps for SignedCertificates.
    fn setup_timestamps(valid_until: u32) -> (u32, u32) {
        (
            unix_u32_now().unwrap(),
            system_unix_time_to_u32(&(SystemTime::now() + Duration::from_secs(valid_until as u64)))
                .unwrap(),
        )
    }

    #[test]
    fn init_signed_certificate() {
        let pubkey = StaticPublicKey::new(Keypair::default().get_public_key());
        let (valid_from, not_valid_after) = setup_timestamps(5);
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &pubkey);

        assert!(cert.is_ok());
    }

    #[test]
    fn invalid_time_signed_certificate() {
        let (valid_from, not_valid_after) = setup_timestamps(5);

        // Should return an error since valid_from time is greater than not_valid_after time.
        let pubkey = StaticPublicKey::new(Keypair::default().get_public_key());
        let cert = SignedCertificate::new(0, not_valid_after, valid_from, &pubkey);

        assert!(cert.is_err());
    }

    // Helper function to setup the keys and a signature noise message for certificate
    // verification.
    fn setup_keys_and_signature() -> (AuthorityKeyPair, StaticPublicKey, SignatureNoiseMessage) {
        let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});

        let static_keypair = StaticKeyPair::default();
        let static_pubkey = static_keypair.get_public_key();

        let (valid_from, not_valid_after) = setup_timestamps(1);
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &static_pubkey).unwrap();

        let signature = authority_sign_cert(&authority_keypair, &cert).unwrap();
        let signature_noise_message = SignatureNoiseMessage::new(&cert, signature);

        (authority_keypair, static_pubkey, signature_noise_message)
    }

    #[test]
    fn expired_certficate_format() {
        let (authority_keypair, static_pub_key, signature_noise_message) =
            setup_keys_and_signature();

        let key = base58::encode_slice(&authority_keypair.get_inner().unwrap().public.to_bytes());

        let certificate: CertificateFormat<AuthorityPublicKey> =
            CertificateFormat::new(&key, &static_pub_key, &signature_noise_message).unwrap();

        // TODO: It would be better if we could mock the system time.
        sleep(Duration::new(1, 0));
        assert!(certificate.verify().is_err())
    }

    #[test]
    fn invalid_pubkey() {
        let (_, static_pub_key, signature_noise_message) = setup_keys_and_signature();
        let invalid_pubkey = "jg9QygGzKSVyxExPrj6bSCDq93c17Krj9yq5kNQnM3GP65";

        let certificate = CertificateFormat::<AuthorityPublicKey>::new(
            invalid_pubkey,
            &static_pub_key,
            &signature_noise_message,
        );

        assert!(matches!(certificate, Err(Error::ParseError { .. })));
    }

    #[test]
    fn noise_nx() {
        // This test contains a simulated lifecycle of the noise handshake
        // including validating the SignatureNoiseMessage.
        let server_static_keypair = StaticKeyPair::default();

        let mut server = NoiseSession::new_noise_responder(Some(server_static_keypair.clone()));
        let mut client = NoiseSession::new_noise_initiator();

        let mut read_buf = [0u8; 1024];

        // -> e - First half of the handshake
        client.send_message(&mut read_buf).unwrap();
        server.recv_message(&mut read_buf).unwrap();

        // <- e... - Second half of the handshake
        server.send_message(&mut read_buf).unwrap();
        client.recv_message(&mut read_buf).unwrap();

        assert!(server.is_transport() && client.is_transport());
        assert_eq!(server.get_handshake_hash(), client.get_handshake_hash());

        // Server generates the SignatureNoiseMessage with a signature over the
        // StaticPublicKey.
        let (valid_from, not_valid_after) = setup_timestamps(100);
        let key = server_static_keypair.get_public_key().get_inner();
        let public_key = StaticPublicKey::new(key);

        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &public_key).unwrap();

        let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});
        let signature_noise_msg =
            SignatureNoiseMessage::from_auth_key(&authority_keypair, &cert).unwrap();

        let mut serialized_signature_msg = Vec::new();
        signature_noise_msg
            .serialize(&mut serialized_signature_msg)
            .unwrap();

        // Copy the serialized signature message into the buffer to simulate
        // sending over the wire.
        let mut buf = [0u8; 1024];
        buf[..serialized_signature_msg.len()].copy_from_slice(&serialized_signature_msg);

        let plain_text = buf.clone();
        server.send_message(&mut buf).unwrap();

        let cipher_text = buf.clone();
        assert!(
            plain_text[..serialized_signature_msg.len()]
                != cipher_text[..serialized_signature_msg.len()]
        );

        // Client reads and decrypts the SignatureNoiseMessage into buf.
        client.recv_message(&mut buf).unwrap();

        assert_eq!(
            buf[..serialized_signature_msg.len()],
            plain_text[..serialized_signature_msg.len()]
        );

        // Client deseializes the SignatureNoiseMessage, builds a CertificateFormat
        // and verifies the signature is from the Mining Pools Authority Keypair.
        let signature_noise_message = deserialize::<SignatureNoiseMessage>(&buf).unwrap();
        let remote_static_key = client.get_remote_static_public_key().unwrap();

        // By Base58 encoding the public authority key, it mimicks the behaviour
        // of the client downloading this from the server pools website or some
        // other public forum.
        let key = base58::encode_slice(&authority_keypair.get_inner().unwrap().public.to_bytes());
        let cert = CertificateFormat::<AuthorityPublicKey>::new(
            &key,
            &remote_static_key,
            &signature_noise_message,
        )
        .unwrap();

        assert!(cert.verify().is_ok());

        // TODO: TMP
        assert_eq!(1, 2);
    }
}
