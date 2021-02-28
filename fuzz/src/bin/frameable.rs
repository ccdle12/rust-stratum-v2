use honggfuzz::fuzz;
use stratumv2::util::deframe_payload;

fn main() {
    fuzz!(|data: &[u8]| {
        deframe_payload(data);
    });
}
