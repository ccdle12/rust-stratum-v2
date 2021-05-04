use honggfuzz::fuzz;
use stratumv2_lib::{
    noise::SignatureNoiseMessage,
    parse::{deserialize, Deserializable},
};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<SignatureNoiseMessage>(&data);
    });
}
