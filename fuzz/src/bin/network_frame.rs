use honggfuzz::fuzz;
use stratumv2::common::NetworkFrame;
use stratumv2::Deserializable;

fn main() {
    fuzz!(|data: &[u8]| {
        NetworkFrame::deserialize(data);
    });
}
