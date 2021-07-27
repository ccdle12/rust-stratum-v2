use crate::codec::Serializable;
use crate::error::{Error, Result};
use crate::noise::signature_noise_message::SignatureNoiseMessage;
use crate::noise::types::{AuthorityPublicKey, StaticPublicKey};
use crate::types::unix_timestamp::unix_u32_now;
use bitcoin::util::base58;
use ed25519_dalek::Verifier;
use std::convert::TryInto;
use std::io;

/// CertificateFormat is used to reconstruct a message to verify a signature
/// given a [SignatureNoiseMessage](struct.SignatureNoiseMessage.html).
///
/// The `verison`, `valid_from` and `not_valid_after` fields are
/// taken from the [SignatureNoiseMessage](struct.SignatureNoiseMessage.html)
/// and the static key of the server to verify the signature was signed with
/// the correct Authority Key.
pub struct CertificateFormat<'a> {
    authority_public_key: AuthorityPublicKey,
    static_public_key: &'a StaticPublicKey,
    signature_noise_message: &'a SignatureNoiseMessage,
}

impl<'a> CertificateFormat<'a> {
    pub fn new(
        authority_public_key: &'a str,
        static_public_key: &'a StaticPublicKey,
        signature_noise_message: &'a SignatureNoiseMessage,
    ) -> Result<CertificateFormat<'a>> {
        // Convert the base58 encoded String into an AuthorityPublicKey object.
        let key_bytes: [u8; 32] = base58::from(authority_public_key)?
            .try_into()
            .map_err(|_| Error::ParseError("Failed to deserialize the base58 public key".into()))?;

        Ok(CertificateFormat {
            authority_public_key: AuthorityPublicKey::from_bytes(&key_bytes)?,
            static_public_key,
            signature_noise_message,
        })
    }

    /// Verify the certificate, specifically the validity of the certificate time
    /// limits and whether the static public key was signed by the AuthorityKeyPair
    /// identifying the Mining Pool.
    pub fn verify(&self) -> Result<()> {
        if unix_u32_now()? >= self.signature_noise_message.not_valid_after {
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

// This implementation of Serializable, intentionally omits the signature field
// in the signature_noise_message. We only need the bytes of the version, valid_from
// and not_valid after to check against the signature and the counter parties
// static public key.
impl Serializable for CertificateFormat<'_> {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok([
            self.signature_noise_message.version.serialize(writer)?,
            self.signature_noise_message.valid_from.serialize(writer)?,
            self.signature_noise_message
                .not_valid_after
                .serialize(writer)?,
            self.static_public_key.serialize(writer)?,
        ]
        .iter()
        .sum())
    }
}
