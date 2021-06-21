use honggfuzz::fuzz;
use stratumv2::{
    noise::SignatureNoiseMessage,
    parse::{deserialize, Deserializable},
};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<SignatureNoiseMessage>(&data);
    });
}
