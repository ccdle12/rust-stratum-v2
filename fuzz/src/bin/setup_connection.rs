use honggfuzz::fuzz;
use stratumv2_lib::mining;
use stratumv2_lib::parse::{deserialize, Deserializable};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<mining::SetupConnectionError>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::SetupConnectionSuccess>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::SetupConnection>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::OpenStandardMiningChannel>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::OpenStandardMiningChannelSuccess>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::OpenStandardMiningChannelError>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::OpenExtendedMiningChannel>(&data);
    });

    fuzz!(|data: &[u8]| {
        deserialize::<mining::OpenExtendedMiningChannelSuccess>(&data);
    });
}
