use honggfuzz::fuzz;
use stratumv2::codec::{deserialize, Deserializable, Message};

fn main() {
    fuzz!(|data: &[u8]| {
        deserialize::<Message>(&data);
    });
}
