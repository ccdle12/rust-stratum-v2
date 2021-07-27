use honggfuzz::fuzz;
use stratumv2::{
    codec::{deserialize, Deserializable},
    noise::SignatureNoiseMessage,
};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<SignatureNoiseMessage>(&data);
    });
}
