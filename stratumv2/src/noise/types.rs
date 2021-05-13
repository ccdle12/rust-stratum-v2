use crate::codec::{ByteParser, Deserializable, Serializable};
use crate::error::Result;
use rand::rngs::OsRng;
use std::convert::TryInto;
use std::io;

/// AuthorityKeyPair is an ed25519_dalek::Keypair used as the Authentication Authority
/// Keypair for the Mining Pool.
pub type AuthorityKeyPair = ed25519_dalek::Keypair;

pub fn generate_authority_keypair() -> AuthorityKeyPair {
    AuthorityKeyPair::generate(&mut OsRng {})
}

/// AuthorityPublicKey is the publicly known key of the
/// [AuthorityKeyPair](struct.AuthorityKeyPair.html) of the Mining Pool.
/// This will be used by the Client to verify the and authenticate the Upstream
/// Node is authorised by the Mining Pool.
pub type AuthorityPublicKey = ed25519_dalek::PublicKey;

/// StaticPublicKey is used as the Noise Diffie-Hellman static public. The key
/// will be signed by the AuthorityKeyPair, to attest to the authenticity of
/// the Mining Pool Server.
pub type StaticPublicKey = noiseexplorer_nx::types::PublicKey;
pub type StaticPrivateKey = noiseexplorer_nx::types::PrivateKey;

impl Serializable for StaticPublicKey {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let public_key = self.as_bytes();
        writer.write(&public_key)?;

        Ok(public_key.len())
    }
}

/// StaticKeyPair is a Keypair used by the responder (Server) as a pre-determined
/// static key that will be signed by the AuthorityKeyPair and used in the
/// [NoiseSession](struct.NoiseSession.html).
///
/// # Examples
///
/// ```rust
/// use stratumv2::noise::StaticKeyPair;
///
///
/// let static_keypair = StaticKeyPair::default();
/// ```
pub type StaticKeyPair = noiseexplorer_nx::types::Keypair;

// TODO: DOC STRING
pub type Signature = ed25519_dalek::Signature;

impl Serializable for Signature {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let public_key = self.to_bytes();
        writer.write(&public_key)?;

        Ok(public_key.len())
    }
}

impl Deserializable for Signature {
    fn deserialize(parser: &mut ByteParser) -> Result<Self> {
        let signature_bytes = parser.next_by(64)?;
        Ok(Signature::new(signature_bytes.try_into()?))
    }
}
