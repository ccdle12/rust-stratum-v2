use crate::error::Error;
use crate::util::{le_bytes_to_u16, le_bytes_to_u32, system_unix_time_to_u32};
use crate::Result;
use crate::{Deserializable, Serializable};
use ed25519_dalek::{Signature, Signer, Verifier};
use std::convert::TryInto;
use std::time::SystemTime;
use std::{io, str};

/// Defines the noise protocol used for secure communication.
pub const NOISE_PROTOCOL: &'static str = "Noise_NX_25519_ChaChaPoly_SHA256";

/// AuthorityKeyPair is an ed25519_dalek::Keypair used as the Authentication Authority
/// Keypair for the Mining Pool.
pub type AuthorityKeyPair = ed25519_dalek::Keypair;

/// AuthorityPublicKey is the publicly known key of the
/// [AuthorityKeyPair](struct.AuthorityKeyPair.html) of the Mining Pool.
/// This will be used by the Client to verify the and authenticate the Upstream
/// Node is authorised by the Mining Pool.
pub type AuthorityPublicKey = ed25519_dalek::PublicKey;

/// StaticPublicKey is used as the Noise DH static public. The key that will be
/// signed by the AuthorityKeyPair, to attest to the authenticity of the Mining Pool
/// Server.
pub type StaticPublicKey = Vec<u8>;

/// Signs a [SignedCertificate](struct.SignatureNoiseMessage.html) using the Mining Pools
/// [AuthorityKeyPair](struct.AuthorityKeyPair.html), authorizing the Upstream Node
/// to operate on behalf of the Mining Pool.
pub fn authority_sign_cert(
    keypair: &AuthorityKeyPair,
    cert: &SignedCertificate,
) -> Result<Signature> {
    let mut signed_cert = Vec::new();
    cert.serialize(&mut signed_cert)?;

    Ok(keypair.sign(&signed_cert))
}

/// A SignedCertificate represents the signed part of a SignatureNoiseMessage.
/// This struct is signed by the Mining Pool's AuthorityKeyPair, attesting to
/// the identity of the StaticPublicKey used in the Noise Diffie-Hellman exchange
/// of the Upstream Node.
pub struct SignedCertificate<'a> {
    version: u16,
    valid_from: u32,
    not_valid_after: u32,
    public_key: &'a StaticPublicKey,
}

impl<'a> SignedCertificate<'a> {
    pub fn new(
        version: u16,
        valid_from: u32,
        not_valid_after: u32,
        public_key: &'a StaticPublicKey,
    ) -> Result<SignedCertificate<'a>> {
        if valid_from >= not_valid_after {
            return Err(Error::RequirementError(
                "the valid_from time cannot be greater than or equal to the not_valid_after time"
                    .into(),
            ));
        }

        if public_key.len() != 32 {
            return Err(Error::RequirementError(
                "the static public key passed is invalid since it's length is greater than 32 bytes"
                    .into(),
            ));
        }

        Ok(SignedCertificate {
            version,
            valid_from,
            not_valid_after,
            public_key,
        })
    }
}
impl Serializable for SignedCertificate<'_> {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // This should NOT serialize the authority_public_key and signature.
        let buffer = serialize!(
            &self.version.to_le_bytes(),
            &self.valid_from.to_le_bytes(),
            &self.not_valid_after.to_le_bytes(),
            &self.public_key
        );

        Ok(writer.write(&buffer)?)
    }
}

/// CertificateFormat is used to reconstruct a message to verify a signature
/// given a [SignatureNoiseMessage](struct.SignatureNoiseMessage.html).
///
/// The `verison`, `valid_from` and `not_valid_after` fields are
/// taken from the [SignatureNoiseMessage](struct.SignatureNoiseMessage.html)
/// and the static key of the server to verify the signature was signed with
/// the correct Authority Key.
pub struct CertificateFormat<'a> {
    authority_public_key: &'a AuthorityPublicKey,
    static_public_key: &'a StaticPublicKey,
    signature_noise_message: &'a SignatureNoiseMessage,
}

impl<'a> CertificateFormat<'a> {
    pub fn new(
        authority_public_key: &'a AuthorityPublicKey,
        static_public_key: &'a StaticPublicKey,
        signature_noise_message: &'a SignatureNoiseMessage,
    ) -> CertificateFormat<'a> {
        CertificateFormat {
            static_public_key,
            authority_public_key,
            signature_noise_message,
        }
    }

    /// Verify the certificate, specifically the validity of the certificate time
    /// limits and whether the static public key was signed by the AuthorityKeyPair
    /// identifying the Mining Pool.
    pub fn verify(&self) -> Result<()> {
        if unix_u32_now!()? >= self.signature_noise_message.not_valid_after {
            return Err(Error::RequirementError(
                "the signature noise message is expired".into(),
            ));
        }

        let mut certificate = Vec::new();
        self.serialize(&mut certificate)?;

        Ok(self
            .authority_public_key
            .verify(&certificate, &self.signature_noise_message.signature)?)
    }
}

impl Serializable for CertificateFormat<'_> {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // This should NOT serialize the authority_public_key and signature.
        let buffer = serialize!(
            &self.signature_noise_message.version.to_le_bytes(),
            &self.signature_noise_message.valid_from.to_le_bytes(),
            &self.signature_noise_message.not_valid_after.to_le_bytes(),
            &self.static_public_key
        );

        Ok(writer.write(&buffer)?)
    }
}

/// SignatureNoiseMessage is sent by the Server after the NX Noise
/// Handshake has completed. The message is used by the Client to reconstruct
/// the full certificate and validate the remote static public key ("s") has
/// been signed by the AuthorityKeyPair of the Mining Pool.
#[derive(Debug, PartialEq)]
pub struct SignatureNoiseMessage {
    version: u16,
    valid_from: u32,
    not_valid_after: u32,
    signature: Signature,
}

impl SignatureNoiseMessage {
    pub fn new(
        version: u16,
        signed_certificate: &SignedCertificate,
        signature: Signature,
    ) -> SignatureNoiseMessage {
        SignatureNoiseMessage {
            version,
            valid_from: signed_certificate.valid_from,
            not_valid_after: signed_certificate.not_valid_after,
            signature,
        }
    }
}

impl Serializable for SignatureNoiseMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = serialize!(
            &self.version.to_le_bytes(),
            &self.valid_from.to_le_bytes(),
            &self.not_valid_after.to_le_bytes(),
            &self.signature.to_bytes()
        );

        Ok(writer.write(&buffer)?)
    }
}

impl Deserializable for SignatureNoiseMessage {
    fn deserialize(bytes: &[u8]) -> Result<SignatureNoiseMessage> {
        // Get the version.
        let start = 0;
        let offset = start + 2;

        let version_bytes = bytes.get(start..offset);
        if version_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing version_bytes in signature noise message".into(),
            ));
        }
        let version = le_bytes_to_u16(version_bytes.unwrap().try_into().unwrap());

        // Get valid from.
        let start = offset;
        let offset = start + 4;

        let valid_from_bytes = bytes.get(start..offset);
        if valid_from_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing valid_from in signature noise message".into(),
            ));
        }
        let valid_from = le_bytes_to_u32(valid_from_bytes.unwrap().try_into().unwrap());

        // Get not_valid_after.
        let start = offset;
        let offset = start + 4;

        let not_valid_after_bytes = bytes.get(start..offset);
        if not_valid_after_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing not_valid_after in signature noise message".into(),
            ));
        }
        let not_valid_after = le_bytes_to_u32(not_valid_after_bytes.unwrap().try_into().unwrap());

        // Get the Signature.
        let start = offset;
        let offset = start + 64;

        let signature_bytes = bytes.get(start..offset);
        if signature_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing signature in signature noise message".into(),
            ));
        }
        let sig_array: [u8; 64] = signature_bytes.unwrap().try_into().unwrap();
        let signature = Signature::new(sig_array);

        Ok({
            SignatureNoiseMessage {
                version,
                valid_from,
                not_valid_after,
                signature,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use snow::Builder;
    use std::thread::sleep;
    use std::time::Duration;

    /// Helper function to generate timestamps for SignedCertificates.
    fn setup_timestamps(valid_until: u32) -> (u32, u32) {
        (
            unix_u32_now!().unwrap(),
            system_unix_time_to_u32(&(SystemTime::now() + Duration::from_secs(valid_until as u64)))
                .unwrap(),
        )
    }

    #[test]
    fn init_signed_certificate() {
        let (valid_from, not_valid_after) = setup_timestamps(5);

        let pubkey = [0u8; 32].to_vec();
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &pubkey);
        assert!(cert.is_ok());
    }

    #[test]
    fn invalid_time_signed_certificate() {
        let (valid_from, not_valid_after) = setup_timestamps(5);

        // Should return an error since valid_from time is greater than not_valid_after time.
        let pubkey = [0u8; 32].to_vec();
        let cert = SignedCertificate::new(0, not_valid_after, valid_from, &pubkey);
        assert!(cert.is_err());
    }

    #[test]
    fn invalid_pubkey_length_signed_certificate() {
        let (valid_from, not_valid_after) = setup_timestamps(5);

        let pubkey = [0u8; 5].to_vec();
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &pubkey);
        assert!(cert.is_err());
    }

    /// Helper function to setup the keys and a signature noise message for certificate
    /// verification.
    fn setup_keys_and_signature() -> (AuthorityKeyPair, StaticPublicKey, SignatureNoiseMessage) {
        let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});

        let static_pub_key = snow::Builder::new(NOISE_PROTOCOL.parse().unwrap())
            .generate_keypair()
            .unwrap()
            .public;

        let (valid_from, not_valid_after) = setup_timestamps(1);
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &static_pub_key).unwrap();

        let signature = authority_sign_cert(&authority_keypair, &cert).unwrap();
        let signature_noise_message = SignatureNoiseMessage::new(0, &cert, signature);

        (authority_keypair, static_pub_key, signature_noise_message)
    }

    #[test]
    fn expired_certficate_format() {
        let (authority_keypair, static_pub_key, signature_noise_message) =
            setup_keys_and_signature();

        let certificate = CertificateFormat::new(
            &authority_keypair.public,
            &static_pub_key,
            &signature_noise_message,
        );

        // TODO: It would be better if we could mock the system time.
        sleep(Duration::new(1, 0));
        assert!(certificate.verify().is_err())
    }

    #[test]
    fn noise_messaging_and_signing() {
        // Build the Mining Pool Server and Keys.
        let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});

        let mut server_msg = [0u8; 1024];
        let server = snow::Builder::new(NOISE_PROTOCOL.parse().unwrap());

        let static_keypair = server.generate_keypair().unwrap();

        let mut server = server
            .local_private_key(&static_keypair.private)
            .build_responder()
            .unwrap();

        // Build the Client.
        let mut client_msg = [0u8; 1024];
        let mut client = snow::Builder::new(NOISE_PROTOCOL.parse().unwrap())
            .build_initiator()
            .unwrap();

        // Send the first message from the client, sending an ephemeral key.
        // The client mixes the ephemeral pubkey to the local symmetric state
        // hash.
        //
        // Noise Act: -> e
        let len = client.write_message(&[], &mut client_msg).unwrap();

        // Server receives the first message `e` and mixes the `e` pubkey to
        // the symmetric state hash.
        server.read_message(&client_msg[..len], &mut []).unwrap();

        // Server responds with their own ephemeral pubkey and performs DH exchange
        // on the ephemeral pubkey exchanged and the servers static key.
        let len = server.write_message(&[], &mut server_msg).unwrap();

        // Client reads the final message from the Server, mixes the keys exchanged
        // to their symmetric state hash and completes the Noise handshake.
        client.read_message(&server_msg[..len], &mut []).unwrap();

        // Assert the handshake is complete for both parties.
        assert!(client.is_handshake_finished() && server.is_handshake_finished());

        // Assert the handshake symmetric state hash is the same on the Server
        // and Client.
        assert_eq!(client.get_handshake_hash(), server.get_handshake_hash());

        // Transition both parties into Transport Mode and send the
        // SignatureNoiseMessage over the secure channel.
        let mut client = client.into_transport_mode().unwrap();
        let mut server = server.into_transport_mode().unwrap();

        // Simulate the Client downloading the Mining Pools AuthorityPublicKey
        // to verify the received SignatureNoiseMessage.
        let authority_pub_key =
            AuthorityPublicKey::from_bytes(authority_keypair.public.as_bytes()).unwrap();

        // Server generates the SignatureNoiseMessage with a signature over the
        // Static Public Key.
        let (valid_from, not_valid_after) = setup_timestamps(100);
        let cert =
            SignedCertificate::new(0, valid_from, not_valid_after, &static_keypair.public).unwrap();

        let signature = authority_sign_cert(&authority_keypair, &cert).unwrap();
        let signature_noise_msg = SignatureNoiseMessage::new(0, &cert, signature);

        let mut serialized_signature_msg = Vec::new();
        signature_noise_msg
            .serialize(&mut serialized_signature_msg)
            .unwrap();

        let mut encrypted_signature_msg = [0u8; 1024];
        let len = server
            .write_message(&serialized_signature_msg, &mut encrypted_signature_msg)
            .unwrap();

        // Client reads and decrypts the SignatureNoiseMessage.
        let mut decrypted_signature_msg = [0u8; 1024];
        client
            .read_message(
                &encrypted_signature_msg[..len],
                &mut decrypted_signature_msg,
            )
            .unwrap();

        // Assert the decrypted message is different from the encrypted message.
        assert!(decrypted_signature_msg != encrypted_signature_msg);

        // Client deseializes the SignatureNoiseMessage, builds a CertificateFormat
        // and verifies the signature is from the Mining Pools Authority Keypair.
        let signature_noise_message =
            SignatureNoiseMessage::deserialize(&decrypted_signature_msg).unwrap();

        let remote_static_key = client.get_remote_static().unwrap().to_vec();
        let cert = CertificateFormat::new(
            &authority_pub_key,
            &remote_static_key,
            &signature_noise_message,
        );

        assert!(cert.verify().is_ok());
    }
}
