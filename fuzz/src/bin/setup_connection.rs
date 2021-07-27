use honggfuzz::fuzz;
use stratumv2::{
    codec::{deserialize, Deserializable},
    mining,
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
