use honggfuzz::fuzz;
use stratumv2::mining;
use stratumv2::Deserializable;

// TODO: SHOULD I SEPARATE THIS TO DIFFERENT FILES?
fn main() {
    fuzz!(|data: &[u8]| {
        mining::SetupConnectionError::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::SetupConnection::deserialize(&data);
    });
}
