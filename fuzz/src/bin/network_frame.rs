use honggfuzz::fuzz;
use stratumv2::frame::Message;
use stratumv2::parse::{deserialize, Deserializable};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<Message>(&data);
    });
}
