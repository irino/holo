//
// Copyright (c) The Holo Core Contributors
//
// SPDX-License-Identifier: MIT
//

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::LazyLock as Lazy;

use holo_bgp::packet::attribute::{
    Aggregator, AsPath, AsPathSegment, Attrs, BaseAttrs, ClusterList, CommList,
};
use holo_bgp::packet::consts::{AsPathSegmentType, Origin};
use holo_bgp::packet::message::{
    Message, MpReachNlri, MpUnreachNlri, ReachNlri, UnreachNlri, UpdateMsg,
};
use holo_utils::bgp::{Comm, ExtComm, Extv6Comm, LargeComm};
use ipnetwork::{Ipv4Network, Ipv6Network};

use super::{test_decode_msg, test_encode_msg};

static UPDATE1: Lazy<(Vec<u8>, Message)> = Lazy::new(|| {
    (
        vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x17, 0x02, 0x00, 0x00, 0x00,
            0x00,
        ],
        Message::Update(UpdateMsg {
            reach: None,
            unreach: None,
            mp_reach: None,
            mp_unreach: None,
            attrs: None,
        }),
    )
});

static UPDATE2: Lazy<(Vec<u8>, Message)> = Lazy::new(|| {
    (
        vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x29, 0x02, 0x00, 0x08, 0x18,
            0x0a, 0x00, 0x01, 0x18, 0x0a, 0x00, 0x02, 0x01, 0x00, 0x90, 0x0e,
            0x00, 0x47, 0x00, 0x02, 0x01, 0x20, 0x30, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x07, 0xbd,
            0x19, 0x11, 0x1c, 0x84, 0x11, 0x00, 0x80, 0x20, 0x01, 0x0d, 0xb8,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x80, 0x20, 0x01, 0x0d, 0xb8, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x90, 0x0f, 0x00, 0x25,
            0x00, 0x02, 0x01, 0x80, 0x20, 0x01, 0x0d, 0xb8, 0x00, 0x02, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x20,
            0x01, 0x0d, 0xb8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x40, 0x01, 0x01, 0x00, 0x50, 0x02, 0x00,
            0x0e, 0x02, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x03, 0x40, 0x03, 0x04, 0x01, 0x01, 0x01, 0x01,
            0x80, 0x04, 0x04, 0x00, 0x00, 0x01, 0xf4, 0x40, 0x05, 0x04, 0x00,
            0x00, 0x01, 0xf4, 0x40, 0x06, 0x00, 0xc0, 0x07, 0x08, 0x00, 0x00,
            0x03, 0xe8, 0x02, 0x02, 0x02, 0x02, 0xd0, 0x08, 0x00, 0x0c, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x80, 0x09, 0x04, 0x01, 0x01, 0x01, 0x01, 0x90, 0x0a, 0x00, 0x04,
            0x03, 0x03, 0x03, 0x03, 0xd0, 0x10, 0x00, 0x08, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x01, 0xd0, 0x19, 0x00, 0x14, 0x20, 0x01,
            0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xd0, 0x20, 0x00, 0x0c,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x20, 0x0a, 0x00, 0xff, 0x01, 0x20, 0x0a, 0x00, 0xff, 0x02,
        ],
        Message::Update(UpdateMsg {
            reach: Some(ReachNlri {
                prefixes: vec![
                    Ipv4Network::from_str("10.0.255.1/32").unwrap(),
                    Ipv4Network::from_str("10.0.255.2/32").unwrap(),
                ],
                nexthop: Ipv4Addr::from_str("1.1.1.1").unwrap(),
            }),
            unreach: Some(UnreachNlri {
                prefixes: vec![
                    Ipv4Network::from_str("10.0.1.0/24").unwrap(),
                    Ipv4Network::from_str("10.0.2.0/24").unwrap(),
                ],
            }),
            mp_reach: Some(MpReachNlri::Ipv6Unicast {
                prefixes: vec![
                    Ipv6Network::from_str("2001:db8:1::1/128").unwrap(),
                    Ipv6Network::from_str("2001:db8:1::2/128").unwrap(),
                ],
                nexthop: Ipv6Addr::from_str("3000::1").unwrap(),
                ll_nexthop: Some(
                    Ipv6Addr::from_str("fe80::4207:bd19:111c:8411").unwrap(),
                ),
            }),
            mp_unreach: Some(MpUnreachNlri::Ipv6Unicast {
                prefixes: vec![
                    Ipv6Network::from_str("2001:db8:2::1/128").unwrap(),
                    Ipv6Network::from_str("2001:db8:2::2/128").unwrap(),
                ],
            }),
            attrs: Some(Attrs {
                base: BaseAttrs {
                    origin: Origin::Igp,
                    as_path: AsPath {
                        segments: [AsPathSegment {
                            seg_type: AsPathSegmentType::Sequence,
                            members: [1, 2, 3].into(),
                        }]
                        .into(),
                    },
                    as4_path: None,
                    nexthop: None,
                    ll_nexthop: None,
                    med: Some(500),
                    local_pref: Some(500),
                    aggregator: Some(Aggregator {
                        asn: 1000,
                        identifier: Ipv4Addr::from_str("2.2.2.2").unwrap(),
                    }),
                    as4_aggregator: None,
                    atomic_aggregate: true,
                    originator_id: Some(Ipv4Addr::from_str("1.1.1.1").unwrap()),
                    cluster_list: Some(ClusterList(
                        [Ipv4Addr::from_str("3.3.3.3").unwrap()].into(),
                    )),
                },
                comm: Some(CommList([Comm(1), Comm(2), Comm(3)].into())),
                ext_comm: Some(CommList(
                    [ExtComm([0, 0, 0, 1, 0, 0, 0, 1])].into(),
                )),
                extv6_comm: Some(CommList(
                    [Extv6Comm(Ipv6Addr::from_str("2001:db8::1").unwrap(), 1)]
                        .into(),
                )),
                large_comm: Some(CommList(
                    [LargeComm([0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1])].into(),
                )),
                unknown: vec![],
            }),
        }),
    )
});

#[test]
fn test_encode_update1() {
    let (ref bytes, ref msg) = *UPDATE1;
    test_encode_msg(bytes, msg);
}

#[test]
fn test_decode_update1() {
    let (ref bytes, ref msg) = *UPDATE1;
    test_decode_msg(bytes, msg);
}

#[test]
fn test_encode_update2() {
    let (ref bytes, ref msg) = *UPDATE2;
    test_encode_msg(bytes, msg);
}

#[test]
fn test_decode_update2() {
    let (ref bytes, ref msg) = *UPDATE2;
    test_decode_msg(bytes, msg);
}
