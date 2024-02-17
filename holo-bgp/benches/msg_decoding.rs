#![feature(lazy_cell)]

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use holo_bgp::neighbor::PeerType;
use holo_bgp::packet::message::{
    Capability, DecodeCxt, FourOctetAsNumber, Message,
};

fn msg_decode(n: u64) {
    let cxt = DecodeCxt {
        peer_type: PeerType::Internal,
        peer_as: n as u32,
        capabilities: [Capability::FourOctetAsNumber {
            asn: FourOctetAsNumber(n as u32),
        }]
        .into(),
    };

    let bytes = vec![
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0x00, 0x3d, 0x01, 0x04, 0x00, 0x01, 0x00, 0xb4,
        0x01, 0x01, 0x01, 0x01, 0x20, 0x02, 0x06, 0x01, 0x04, 0x00, 0x01, 0x00,
        0x01, 0x02, 0x06, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x02, 0x02, 0x02,
        0x00, 0x02, 0x06, 0x41, 0x04, 0x00, 0x01, 0x00, 0x0e, 0x02, 0x02, 0x46,
        0x00,
    ];

    for _ in 0..n {
        let _msg = Message::decode(&bytes, &cxt).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Message decode", |b| {
        b.iter(|| msg_decode(black_box(10000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
