// TODO: Fix up the errors to be noise crate scoped
use crate::types::{AuthKey, Signature, StaticPublicKey};
use std::io;
use stratumv2_serde::Serializable;

/// A SignedCertificate represents the signed part of a SignatureNoiseMessage.
/// This struct is signed by the Mining Pool's AuthorityKeyPair, attesting to
/// the identity of the StaticPublicKey used in the Noise Diffie-Hellman exchange
/// of the Upstream Node.
pub struct SignedCertificate<'a> {
    pub version: u16,
    pub valid_from: u32,
    pub not_valid_after: u32,
    pub public_key: &'a StaticPublicKey,
}

impl<'a> SignedCertificate<'a> {
    pub fn new(
        version: u16,
        valid_from: u32,
        not_valid_after: u32,
        public_key: &'a StaticPublicKey,
    ) -> Result<SignedCertificate<'a>, stratumv2_serde::Error> {
        if valid_from >= not_valid_after {
            return Err(stratumv2_serde::Error::RequirementError(
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
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize, stratumv2_serde::Error> {
        Ok([
            self.version.serialize(writer)?,
            self.valid_from.serialize(writer)?,
            self.not_valid_after.serialize(writer)?,
            self.public_key.serialize(writer)?,
        ]
        .iter()
        .sum())
    }
}

/// Signs a [SignedCertificate](struct.SignedCertificate.html) using the Mining Pools
/// [AuthorityKeyPair](struct.AuthorityKeyPair.html), authorizing the Upstream Node
/// to operate on behalf of the Mining Pool.
pub fn authority_sign_cert<T: AuthKey>(
    keypair: &T,
    cert: &SignedCertificate,
) -> Result<Signature, stratumv2_serde::Error> {
    let mut signed_cert = Vec::new();
    cert.serialize(&mut signed_cert)?;
    Ok(keypair.sign(&signed_cert))
}
