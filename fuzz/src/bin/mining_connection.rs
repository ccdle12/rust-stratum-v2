use honggfuzz::fuzz;
use stratumv2::mining;
use stratumv2::Deserializable;

fn main() {
    fuzz!(|data: &[u8]| {
        mining::UpdateChannel::deserialize(&data);
    });
}
