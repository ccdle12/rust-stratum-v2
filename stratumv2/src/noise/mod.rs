use crate::error::Error;
use crate::util::{le_bytes_to_u16, le_bytes_to_u32, system_unix_time_to_u32};
use crate::Result;
use crate::{Deserializable, Serializable};
use ed25519_dalek::{Signature, Signer, Verifier};
use noiseexplorer_nx::types::Keypair;
use std::convert::TryInto;
use std::time::SystemTime;
use std::{io, str};

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
pub type StaticPublicKey = noiseexplorer_nx::types::PublicKey;

/// NoiseSession is a struct that contains all the state required to handle a
/// key exchange and subsequent encrypted communication.
pub type NoiseSession = noiseexplorer_nx::noisesession::NoiseSession;

/// StaticKeyPair is a Keypair used by the responder (Server) to use a pre-determined
/// static key that will be signed by the AuthorityKeyPair.
pub type StaticKeyPair = noiseexplorer_nx::types::Keypair;

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

/// Creates a NoiseSession for a responder, this will usually be the Upstream
/// Node (Server) with the option of using a pre-determined StaticKeyPair.
pub fn new_noise_responder(static_keypair: Option<StaticKeyPair>) -> NoiseSession {
    let key = match static_keypair {
        Some(k) => k,
        None => Keypair::default(),
    };

    NoiseSession::init_session(false, &[], key)
}

/// Creates a NoiseSession for an initiator, this will usually be the Client.
pub fn new_noise_initiator() -> NoiseSession {
    NoiseSession::init_session(true, &[], Keypair::default())
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
            &self.public_key.as_bytes()
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
            &self.static_public_key.as_bytes()
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
    pub version: u16,
    pub valid_from: u32,
    pub not_valid_after: u32,
    pub signature: Signature,
}

impl SignatureNoiseMessage {
    pub fn new(cert: &SignedCertificate, signature: Signature) -> SignatureNoiseMessage {
        SignatureNoiseMessage {
            version: cert.version,
            valid_from: cert.valid_from,
            not_valid_after: cert.not_valid_after,
            signature,
        }
    }

    /// from_auth_key generates a signature from an AuthorityKeyPair over the
    /// SignedCertificate and returns a SignatureNoiseMessage.
    pub fn from_auth_key(
        authority_keypair: &AuthorityKeyPair,
        cert: &SignedCertificate,
    ) -> Result<SignatureNoiseMessage> {
        Ok(SignatureNoiseMessage {
            version: cert.version,
            valid_from: cert.valid_from,
            not_valid_after: cert.not_valid_after,
            signature: authority_sign_cert(&authority_keypair, &cert)?,
        })
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
    use noiseexplorer_nx::types::Keypair;
    use rand::rngs::OsRng;
    use std::thread::sleep;
    use std::time::Duration;

    // /// Helper function to generate timestamps for SignedCertificates.
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

        let pubkey = Keypair::default().get_public_key();
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &pubkey);
        assert!(cert.is_ok());
    }

    #[test]
    fn invalid_time_signed_certificate() {
        let (valid_from, not_valid_after) = setup_timestamps(5);

        // Should return an error since valid_from time is greater than not_valid_after time.
        let pubkey = Keypair::default().get_public_key();
        let cert = SignedCertificate::new(0, not_valid_after, valid_from, &pubkey);
        assert!(cert.is_err());
    }

    /// Helper function to setup the keys and a signature noise message for certificate
    /// verification.
    fn setup_keys_and_signature() -> (AuthorityKeyPair, StaticPublicKey, SignatureNoiseMessage) {
        let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});

        let static_keypair = StaticKeyPair::default();
        let static_pub_key = static_keypair.get_public_key();

        let (valid_from, not_valid_after) = setup_timestamps(1);
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &static_pub_key).unwrap();

        let signature = authority_sign_cert(&authority_keypair, &cert).unwrap();
        let signature_noise_message = SignatureNoiseMessage::new(&cert, signature);

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
    fn noise_nx() {
        let server_static_keypair = StaticKeyPair::default();

        let mut server = new_noise_responder(Some(server_static_keypair.clone()));
        let mut client = new_noise_initiator();

        let mut read_buf = [0u8; 1024];

        // -> e
        client.send_message(&mut read_buf).unwrap();
        server.recv_message(&mut read_buf).unwrap();

        // <- e...
        server.send_message(&mut read_buf).unwrap();
        client.recv_message(&mut read_buf).unwrap();

        assert!(server.is_transport() && client.is_transport());
        assert_eq!(server.get_handshake_hash(), client.get_handshake_hash());

        // Server generates the SignatureNoiseMessage with a signature over the
        // StaticPublicKey.
        let (valid_from, not_valid_after) = setup_timestamps(100);
        let public_key = &server_static_keypair.get_public_key();
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, public_key).unwrap();

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
        let signature_noise_message = SignatureNoiseMessage::deserialize(&buf).unwrap();
        let remote_static_key = client.get_remote_static_public_key().unwrap();

        let cert = CertificateFormat::new(
            &authority_keypair.public,
            &remote_static_key,
            &signature_noise_message,
        );

        assert!(cert.verify().is_ok());
    }
}
