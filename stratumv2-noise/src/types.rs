use crate::error::Result as NoiseResult;
use ed25519_dalek::{Signer, Verifier};
use noiseexplorer_nx::types::{Keypair as NxKeypair, PublicKey as NxPublicKey};
use rand::{CryptoRng, RngCore};
use std::{clone::Clone, convert::TryInto, io};
use stratumv2_serde::{ByteParser, Deserializable, Serializable};

pub trait AuthKey {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self;
    fn sign(&self, signed_cert: &[u8]) -> Signature;
}

/// AuthorityKeyPair is an ed25519_dalek::Keypair used as the Authentication Authority
/// Keypair for the Mining Pool.
pub struct AuthorityKeyPair {
    inner: ed25519_dalek::Keypair,
}

impl AuthorityKeyPair {
    pub fn get_inner(&self) -> NoiseResult<ed25519_dalek::Keypair> {
        Ok(self.clone().inner)
    }
}

impl AuthKey for AuthorityKeyPair {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> AuthorityKeyPair {
        AuthorityKeyPair {
            inner: ed25519_dalek::Keypair::generate(rng),
        }
    }

    fn sign(&self, signed_cert: &[u8]) -> Signature {
        let sig = self.inner.sign(&signed_cert);
        Signature { inner: sig }
    }
}

impl Clone for AuthorityKeyPair {
    fn clone(&self) -> Self {
        let bytes = self.inner.to_bytes();
        AuthorityKeyPair {
            inner: ed25519_dalek::Keypair::from_bytes(&bytes).unwrap(),
        }
    }
}

pub trait AuthPubKey {
    fn verify(&self, certificate: &[u8], signature: &Signature) -> NoiseResult<()>;
    fn from_bytes(pubkey: &[u8]) -> NoiseResult<Self>
    where
        Self: Sized;
}

/// AuthorityPublicKey is the publicly known key of the
/// [AuthorityKeyPair](struct.AuthorityKeyPair.html) of the Mining Pool.
/// This will be used by the Client to verify the and authenticate the Upstream
/// Node is authorised by the Mining Pool.
pub struct AuthorityPublicKey {
    inner: ed25519_dalek::PublicKey,
}

impl AuthPubKey for AuthorityPublicKey {
    fn verify(&self, certificate: &[u8], signature: &Signature) -> NoiseResult<()> {
        Ok(self.inner.verify(&certificate, &signature.get_inner())?)
    }

    fn from_bytes(pubkey: &[u8]) -> NoiseResult<AuthorityPublicKey> {
        Ok(AuthorityPublicKey {
            inner: ed25519_dalek::PublicKey::from_bytes(pubkey)?,
        })
    }
}

/// StaticPublicKey is used as the Noise Diffie-Hellman static public. The key
/// will be signed by the AuthorityKeyPair, to attest to the authenticity of
/// the Mining Pool Server.
pub struct StaticPublicKey {
    inner: NxPublicKey,
}

impl StaticPublicKey {
    pub fn new(inner: NxPublicKey) -> Self {
        StaticPublicKey { inner }
    }

    pub fn get_inner(&self) -> NxPublicKey {
        self.inner
    }
}

impl Serializable for StaticPublicKey {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize, stratumv2_serde::Error> {
        let public_key = self.inner.as_bytes();
        writer.write(&public_key)?;

        Ok(public_key.len())
    }
}

pub trait StaticKey {
    fn default() -> Self;
    fn get_public_key(&self) -> StaticPublicKey;
}

/// StaticKeyPair is a Keypair used by the responder (Server) as a pre-determined
/// static key that will be signed by the AuthorityKeyPair and used in the
/// [NoiseSession](struct.NoiseSession.html).
#[derive(Clone, PartialEq)]
pub struct StaticKeyPair {
    inner: NxKeypair,
}

impl StaticKeyPair {
    pub fn get_inner(&self) -> NxKeypair {
        self.inner.clone()
    }
}

impl StaticKey for StaticKeyPair {
    fn default() -> Self {
        StaticKeyPair {
            inner: NxKeypair::default(),
        }
    }

    fn get_public_key(&self) -> StaticPublicKey {
        StaticPublicKey {
            inner: self.inner.get_public_key(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Signature {
    inner: ed25519_dalek::Signature,
}

impl Signature {
    pub fn new(bytes: &[u8]) -> Result<Self, stratumv2_serde::Error> {
        Ok(Signature {
            inner: ed25519_dalek::Signature::new(bytes.try_into()?),
        })
    }

    pub fn get_inner(&self) -> ed25519_dalek::Signature {
        self.inner
    }
}

impl Serializable for Signature {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize, stratumv2_serde::Error> {
        let public_key = self.inner.to_bytes();
        writer.write(&public_key)?;

        Ok(public_key.len())
    }
}

impl Deserializable for Signature {
    fn deserialize(parser: &mut ByteParser) -> Result<Self, stratumv2_serde::Error> {
        let signature_bytes = parser.next_by(64)?;
        Signature::new(signature_bytes)
    }
}
