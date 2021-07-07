use honggfuzz::fuzz;
use stratumv2::network::{ConnectionEncryptor, Encryptor};

fn main() {
    let mut receiver = ConnectionEncryptor::new_inbound();
    fuzz!(|data: &[u8]| {
        receiver.recv_handshake(&mut data.to_vec());
    });

    let mut initiator = ConnectionEncryptor::new_outbound();
    fuzz!(|data: &[u8]| {
        initiator.recv_handshake(&mut data.to_vec());
    });
}
