use honggfuzz::fuzz;
use stratumv2::mining;
use stratumv2::Deserializable;

fn main() {
    fuzz!(|data: &[u8]| {
        mining::SetupConnectionError::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::SetupConnectionSuccess::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::SetupConnection::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::OpenStandardMiningChannel::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::OpenStandardMiningChannelSuccess::deserialize(&data);
    });

    fuzz!(|data: &[u8]| {
        mining::OpenStandardMiningChannelError::deserialize(&data);
    });
}
