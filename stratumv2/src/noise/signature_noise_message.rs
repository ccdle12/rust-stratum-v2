use crate::{
    error::Result,
    noise::{
        signed_certificate::{authority_sign_cert, SignedCertificate},
        types::AuthorityKeyPair,
        types::Signature,
    },
    parse::{ByteParser, Deserializable, Serializable},
};
use std::io;

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
    // TODO: EXAMPLE
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
        Ok([
            self.version.serialize(writer)?,
            self.valid_from.serialize(writer)?,
            self.not_valid_after.serialize(writer)?,
            self.signature.serialize(writer)?,
        ]
        .iter()
        .sum())
    }
}

impl Deserializable for SignatureNoiseMessage {
    fn deserialize(parser: &mut ByteParser) -> Result<SignatureNoiseMessage> {
        let version = u16::deserialize(parser)?;
        let valid_from = u32::deserialize(parser)?;
        let not_valid_after = u32::deserialize(parser)?;
        let signature = Signature::deserialize(parser)?;

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
