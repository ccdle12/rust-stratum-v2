use stratumv2_lib::noise::{AuthorityKeyPair, StaticKeyPair};

// TODO:
pub struct PoolServer<'a> {
    // TODO:
    authority_keypair: Option<&'a AuthorityKeyPair>,

    // TODO:
    static_keypair: Option<&'a StaticKeyPair>,
}

impl<'a> PoolServer<'a> {
    // TODO:
    pub fn new(
        authority_keypair: Option<&'a AuthorityKeyPair>,
        static_keypair: Option<&'a StaticKeyPair>,
    ) -> PoolServer<'a> {
        PoolServer {
            authority_keypair,
            static_keypair,
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
