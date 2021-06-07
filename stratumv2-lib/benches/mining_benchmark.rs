use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stratumv2_lib::{
    mining::{
        OpenExtendedMiningChannel, OpenExtendedMiningChannelError, OpenMiningChannelErrorCode,
    },
    parse::{deserialize, serialize},
    types::U256,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("OpenExtendedMiningChannel serde", |b| {
        b.iter(|| {
            let msg = OpenExtendedMiningChannel::new(
                black_box(1u32),
                black_box("user id"),
                black_box(3.0f32),
                black_box(U256([4u8; 32])),
                black_box(5u16),
            )
            .unwrap();

            let ser = serialize(&msg).unwrap();
            deserialize::<OpenExtendedMiningChannel>(&ser).unwrap();
        })
    });

    c.bench_function("OpenExtendedMiningChannelError serde", |b| {
        b.iter(|| {
            let msg = OpenExtendedMiningChannelError::new(
                black_box(1),
                black_box(OpenMiningChannelErrorCode::UnknownUser),
            )
            .unwrap();

            let ser = serialize(&msg).unwrap();
            deserialize::<OpenExtendedMiningChannelError>(&ser).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
