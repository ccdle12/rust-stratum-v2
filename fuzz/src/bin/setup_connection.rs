use honggfuzz::fuzz;
use stratumv2_lib::{
    mining,
    parse::{deserialize, Deserializable},
};

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
}
