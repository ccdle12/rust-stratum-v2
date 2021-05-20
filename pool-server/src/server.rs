use stratumv2_lib::{
    noise::{AuthorityKeyPair, SignatureNoiseMessage, SignedCertificate, StaticKeyPair},
    types::{unix_u32_now, unix_u32_one_year_from_now},
};

// TODO:
pub struct PoolServer<'a> {
    // TODO:
    pub(crate) authority_keypair: Option<&'a AuthorityKeyPair>,

    // TODO:
    pub(crate) static_keypair: &'a StaticKeyPair,

    // TODO:
    pub(crate) signature_noise_message: Option<SignatureNoiseMessage>,
}

impl<'a> PoolServer<'a> {
    // TODO:
    pub fn new(
        authority_keypair: Option<&'a AuthorityKeyPair>,
        static_keypair: &'a StaticKeyPair,
    ) -> PoolServer<'a> {
        // TODO: Handle these errors
        // NOTE: For now generate the SignedCertificate using default values.
        let signed_cert = SignedCertificate::new(
            0,
            unix_u32_now().unwrap(),
            unix_u32_one_year_from_now().unwrap(),
            &static_keypair.public_key,
        )
        .unwrap();

        // NOTE: I'm not sure what would be the default behaviour but for now,
        // generate a SignatureNoiseMessage that can be sent to clients if
        // the Server loads the AuthorityKeypair into the Server.
        let signature_noise_message = match &authority_keypair {
            Some(a) => Some(SignatureNoiseMessage::from_auth_key(a, &signed_cert).unwrap()),
            _ => None,
        };

        PoolServer {
            authority_keypair,
            static_keypair,
            signature_noise_message,
        }
    }

    // TODO:
    pub fn listen(self) {
        // TODO:
        loop {
            println!("foo");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use stratumv2_lib::noise::generate_authority_keypair;

    #[test]
    fn init_pool_server() {
        let authority_keypair = generate_authority_keypair();
        let static_keypair = StaticKeyPair::default();

        let pool_server = PoolServer::new(Some(&authority_keypair), &static_keypair);

        assert!(pool_server.signature_noise_message.is_some());
        assert!(pool_server.authority_keypair.is_some());
    }
}
