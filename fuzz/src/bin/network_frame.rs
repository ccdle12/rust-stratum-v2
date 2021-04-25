use honggfuzz::fuzz;
use stratumv2_lib::frame::Message;
use stratumv2_lib::parse::{deserialize, Deserializable};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<Message>(&data);
    });
}
