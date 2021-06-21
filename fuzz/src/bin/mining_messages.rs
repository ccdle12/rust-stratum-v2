use honggfuzz::fuzz;
use stratumv2::{
    mining,
    parse::{deserialize, Deserializable},
};

fn main() {
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

    fuzz!(|data: &[u8]| {
        deserialize::<mining::UpdateChannel>(&data);
    });
}
