use honggfuzz::fuzz;
use stratumv2::noise;
use stratumv2::Deserializable;

fn main() {
    fuzz!(|data: &[u8]| {
        noise::SignatureNoiseMessage::deserialize(&data);
    });
}
