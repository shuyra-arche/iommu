//! Independent provider-owned golden bytes and boundaries for the IOMMU API.
//!
//! These cases use only the public `iommu_api` surface and the published
//! `env`/`wire`/`hub-api` crates. They do not install a Kernel, call runtime
//! syscalls, or fake a Kernel host. The production API files stay byte-identical
//! to Source; if a golden disagreed with them, the protocol spec would be the
//! thing to inspect, never the test.

use env::{PieToken, TaskId};
use iommu_api as api;
use api::{
    ACQUIRE, Acquire, BACK, Call, ESTABLISH, ENTRY, ENTRY_NAME, ENTRY_PATH, Establish, Fail,
    INVALIDATE, PARTY, PUBLISH, QUIESCE, RELEASE, Said, Step, Wire, code_to_fail, fail_to_code,
};
use wire::Message;

fn wire_bytes(wire: &Wire) -> ([u8; Wire::LEN], usize) {
    let mut bytes = [0xa5u8; Wire::LEN];
    let end = wire.store(&mut bytes).expect("encodes");
    (bytes, end)
}

#[test]
fn establish_golden_is_exactly_13_bytes_and_rejects_wrong_opcode_truncation_or_trailing() {
    let frame = Establish {
        op: ESTABLISH,
        requester: 0x44332211,
        back: PieToken::mint(0x8877665544332211),
    };
    let golden: [u8; Establish::LEN] = [
        0x06, 0x11, 0x22, 0x33, 0x44, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ];
    let (bytes, end) = wire_bytes(&Wire::Establish(frame));
    assert_eq!(end, 13);
    assert_eq!(&bytes[..end], &golden);
    assert_eq!(Wire::take(&bytes[..end]), Some(Wire::Establish(frame)));
    assert_eq!(Wire::take(&bytes[..end - 1]), None, "truncation");
    assert_eq!(Wire::take(&bytes[..end + 1]), None, "trailing byte");
    assert_eq!(Wire::take(&bytes), None, "buffer longer than the frame");

    let mut padded = [0u8; Wire::LEN];
    padded[..end].copy_from_slice(&bytes[..end]);
    assert_eq!(Wire::take(&padded), None, "zero padding is not the frame");

    for op in [0, ACQUIRE, PUBLISH, INVALIDATE, QUIESCE, RELEASE, 7, 255] {
        let mut candidate = golden;
        candidate[0] = op;
        assert_eq!(Wire::take(&candidate), None, "opcode {op}");
    }
    assert_eq!(Wire::take(&[]), None);
    assert_eq!(Wire::take(&[0u8]), None);
}

#[test]
fn acquire_golden_is_exactly_61_bytes_and_keeps_opaque_zero_and_112_keys() {
    let frame = Acquire {
        op: ACQUIRE,
        engine: TaskId::new(0x0102030405060708),
        key: 0x1112131415161718,
        hub: TaskId::new(0x2122232425262728),
        serial: 0x3132333435363738,
        execution: TaskId::new(0x4142434445464748),
        source: PieToken::mint(0x5152535455565758),
        access: 0x61626364,
        back: PieToken::mint(0x7172737475767778),
    };
    let golden: [u8; Acquire::LEN] = [
        0x01, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13,
        0x12, 0x11, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x38, 0x37, 0x36, 0x35, 0x34,
        0x33, 0x32, 0x31, 0x48, 0x47, 0x46, 0x45, 0x44, 0x43, 0x42, 0x41, 0x58, 0x57, 0x56, 0x55,
        0x54, 0x53, 0x52, 0x51, 0x64, 0x63, 0x62, 0x61, 0x78, 0x77, 0x76, 0x75, 0x74, 0x73, 0x72,
        0x71,
    ];
    let (bytes, end) = wire_bytes(&Wire::Acquire(frame));
    assert_eq!(end, 61);
    assert_eq!(&bytes[..end], &golden);
    assert_eq!(Wire::take(&bytes[..end]), Some(Wire::Acquire(frame)));
    assert_eq!(Wire::take(&bytes[..end - 1]), None, "truncation");
    assert_eq!(Wire::take(&bytes[..end + 1]), None, "trailing byte");
    assert_eq!(Wire::take(&bytes), None, "buffer longer than the frame");

    for op in [0, ESTABLISH, PUBLISH, INVALIDATE, QUIESCE, RELEASE, 7, 255] {
        let mut candidate = golden;
        candidate[0] = op;
        assert_eq!(Wire::take(&candidate), None, "opcode {op}");
    }

    let zero = Acquire {
        op: ACQUIRE,
        engine: TaskId::new(0),
        key: 0,
        hub: TaskId::new(0),
        serial: 0,
        execution: TaskId::new(0),
        source: PieToken::NONE,
        access: 0,
        back: PieToken::NONE,
    };
    let (zero_bytes, zero_end) = wire_bytes(&Wire::Acquire(zero));
    assert_eq!(zero_end, 61);
    assert_eq!(zero_bytes[0], ACQUIRE);
    assert!(zero_bytes[1..zero_end].iter().all(|byte| *byte == 0));
    assert_eq!(
        Wire::take(&zero_bytes[..zero_end]),
        Some(Wire::Acquire(zero)),
        "zero opaque key is a value, not a rejection"
    );

    let key_112 = Acquire { key: 112, ..zero };
    let (key_bytes, key_end) = wire_bytes(&Wire::Acquire(key_112));
    assert_eq!(key_end, 61);
    assert_eq!(key_bytes[9], 112);
    assert_eq!(
        Wire::take(&key_bytes[..key_end]),
        Some(Wire::Acquire(key_112)),
        "opaque key 112 is not structurally banned"
    );

    assert_eq!(
        core::mem::size_of::<usize>(),
        8,
        "these host cases exercise full-width 64-bit opaque ids"
    );
    let full_token = PieToken::from_bytes(&u64::MAX.to_le_bytes()).unwrap();
    let full = Acquire {
        op: ACQUIRE,
        engine: TaskId::new(usize::MAX),
        key: u64::MAX,
        hub: TaskId::new(usize::MAX),
        serial: u64::MAX,
        execution: TaskId::new(usize::MAX),
        source: full_token,
        access: u32::MAX,
        back: full_token,
    };
    let mut full_golden = [0xffu8; Acquire::LEN];
    full_golden[0] = ACQUIRE;
    let (full_bytes, full_end) = wire_bytes(&Wire::Acquire(full));
    assert_eq!(full_end, 61);
    assert_eq!(&full_bytes[..full_end], &full_golden);
    assert_eq!(
        Wire::take(&full_bytes[..full_end]),
        Some(Wire::Acquire(full)),
        "full-width Domain key, Hub id, serial, execution id, and tokens survive"
    );
}

#[test]
fn step_golden_is_exactly_42_bytes_for_all_four_step_opcodes() {
    let base = Step {
        op: PUBLISH,
        engine: TaskId::new(0x0102030405060708),
        key: 0x1112131415161718,
        hub: TaskId::new(0x2122232425262728),
        serial: 0x3132333435363738,
        reason: 0x42,
        back: PieToken::mint(0x5152535455565758),
    };
    let golden: [u8; Step::LEN] = [
        0x02, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13,
        0x12, 0x11, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x38, 0x37, 0x36, 0x35, 0x34,
        0x33, 0x32, 0x31, 0x42, 0x58, 0x57, 0x56, 0x55, 0x54, 0x53, 0x52, 0x51,
    ];
    for op in [PUBLISH, INVALIDATE, QUIESCE, RELEASE] {
        let frame = Step { op, ..base };
        let wire = match op {
            PUBLISH => Wire::Publish(frame),
            INVALIDATE => Wire::Invalidate(frame),
            QUIESCE => Wire::Quiesce(frame),
            RELEASE => Wire::Release(frame),
            _ => unreachable!(),
        };
        let (bytes, end) = wire_bytes(&wire);
        let mut expected = golden;
        expected[0] = op;
        assert_eq!(end, 42);
        assert_eq!(&bytes[..end], &expected);
        assert_eq!(Wire::take(&bytes[..end]), Some(wire));
        assert_eq!(Wire::take(&bytes[..end - 1]), None, "truncation");
        assert_eq!(Wire::take(&bytes[..end + 1]), None, "trailing byte");
        assert_eq!(Wire::take(&bytes), None, "buffer longer than the frame");

        for other in [PUBLISH, INVALIDATE, QUIESCE, RELEASE] {
            if other != op {
                let mut candidate = bytes;
                candidate[0] = other;
                let other_frame = Step { op: other, ..frame };
                let other_wire = match other {
                    PUBLISH => Wire::Publish(other_frame),
                    INVALIDATE => Wire::Invalidate(other_frame),
                    QUIESCE => Wire::Quiesce(other_frame),
                    RELEASE => Wire::Release(other_frame),
                    _ => unreachable!(),
                };
                assert_eq!(
                    Wire::take(&candidate[..end]),
                    Some(other_wire),
                    "all four step opcodes share one 42-byte shape"
                );
            }
        }
        for other in [0, ACQUIRE, ESTABLISH, 7, 255] {
            let mut candidate = bytes;
            candidate[0] = other;
            assert_eq!(
                Wire::take(&candidate[..end]),
                None,
                "non-step opcode {other} cannot claim a {op} body"
            );
        }
    }
}

#[test]
fn said_golden_is_38_bytes_in_a_48_byte_response_buffer() {
    let said = Said {
        status: 0xff,
        pending: 0xff,
        held: u32::MAX,
        engine: TaskId::new(usize::MAX),
        key: u64::MAX,
        iova: u64::MAX,
        bytes: u64::MAX,
    };
    let golden = [0xffu8; Said::LEN];
    let mut bytes = [0u8; 48];
    let end = said.store(&mut bytes).unwrap();
    assert_eq!(end, 38);
    assert_eq!(&bytes[..end], &golden);
    assert_eq!(<Said as Message>::fetch(&bytes[..end]), Some(said));
    assert_eq!(<Said as Message>::fetch(&bytes[..end - 1]), None);
    assert_eq!(<Said as Message>::fetch(&bytes[..end + 1]), None);
    assert_eq!(
        <Said as Message>::fetch(&bytes),
        None,
        "the 48-byte receive buffer is capacity, not frame length"
    );
    assert_eq!(<Said as Message>::MAX, 48);
    assert_eq!(Said::EMPTY.len(), 48);

    let mut none = [0xa5u8; 48];
    let none_end = Said::NONE.store(&mut none).unwrap();
    assert_eq!(none_end, 38);
    assert!(none[..none_end].iter().all(|byte| *byte == 0));
    assert_eq!(
        <Said as Message>::fetch(&none[..none_end]),
        Some(Said::NONE),
        "the all-zero opaque reply decodes as a value"
    );
}

#[test]
fn fail_codes_are_bidirectional_with_the_bad_fallback() {
    assert_eq!(fail_to_code(None), wire::OK);
    for (fail, code) in [
        (Fail::Unknown, 1),
        (Fail::Conflict, 2),
        (Fail::Denied, 3),
        (Fail::Unavailable, 4),
        (Fail::Bad, 5),
        (Fail::NotReady, 6),
        (Fail::Full, 7),
    ] {
        assert_eq!(fail_to_code(Some(fail)), code);
        assert_eq!(code_to_fail(code), Some(fail));
    }
    assert_eq!(code_to_fail(wire::OK), None);
    for code in [8, 9, 0x7f, 0xff] {
        assert_eq!(code_to_fail(code), Some(Fail::Bad));
    }
}

#[test]
fn marks_entry_back_party_and_call_reply_tokens_are_the_published_contract() {
    assert_eq!(ENTRY, iommu_declaration::marks::ENTRY);
    assert_eq!(ENTRY_NAME, "entry");
    assert_eq!(ENTRY_PATH, "iommu/entry");
    assert_eq!(BACK, env::Mark::of("iommu-back"));
    assert_eq!(PARTY, hub_api::PARTY_IOMMU);
    assert_eq!(
        (ACQUIRE, PUBLISH, INVALIDATE, QUIESCE, RELEASE, ESTABLISH),
        (1, 2, 3, 4, 5, 6)
    );
    assert_eq!(Call::BACK, BACK);

    let establish = Establish {
        op: ESTABLISH,
        requester: 0x44332211,
        back: PieToken::mint(0x0101010101010101),
    };
    let acquire = Acquire {
        op: ACQUIRE,
        engine: TaskId::new(5),
        key: 7,
        hub: TaskId::new(3),
        serial: 11,
        execution: TaskId::new(9),
        source: PieToken::mint(42),
        access: 6,
        back: PieToken::mint(0x0202020202020202),
    };
    let publish = Step {
        op: PUBLISH,
        engine: TaskId::new(5),
        key: 7,
        hub: TaskId::new(3),
        serial: 11,
        reason: 0,
        back: PieToken::mint(0x0303030303030303),
    };
    let invalidate = Step {
        op: INVALIDATE,
        back: PieToken::mint(0x0404040404040404),
        ..publish
    };
    let quiesce = Step {
        op: QUIESCE,
        back: PieToken::mint(0x0505050505050505),
        ..publish
    };
    let release = Step {
        op: RELEASE,
        reason: 2,
        back: PieToken::mint(0x0606060606060606),
        ..publish
    };

    let cases = [
        (Wire::Establish(establish), establish.back),
        (Wire::Acquire(acquire), acquire.back),
        (Wire::Publish(publish), publish.back),
        (Wire::Invalidate(invalidate), invalidate.back),
        (Wire::Quiesce(quiesce), quiesce.back),
        (Wire::Release(release), release.back),
    ];
    for (wire, back) in cases {
        assert_eq!(Call::back(&wire), back);
    }
}

#[test]
fn all_six_opcodes_dispatch_only_to_their_own_frame_shape() {
    let establish = Establish {
        op: ESTABLISH,
        requester: 9,
        back: PieToken::mint(1),
    };
    let acquire = Acquire {
        op: ACQUIRE,
        engine: TaskId::new(5),
        key: 7,
        hub: TaskId::new(3),
        serial: 11,
        execution: TaskId::new(9),
        source: PieToken::mint(42),
        access: 6,
        back: PieToken::mint(2),
    };
    let publish = Step {
        op: PUBLISH,
        engine: TaskId::new(5),
        key: 7,
        hub: TaskId::new(3),
        serial: 11,
        reason: 0,
        back: PieToken::mint(3),
    };
    let cases = [
        (Wire::Establish(establish), ESTABLISH),
        (Wire::Acquire(acquire), ACQUIRE),
        (Wire::Publish(publish), PUBLISH),
        (Wire::Invalidate(Step { op: INVALIDATE, ..publish }), INVALIDATE),
        (Wire::Quiesce(Step { op: QUIESCE, ..publish }), QUIESCE),
        (Wire::Release(Step { op: RELEASE, reason: 2, ..publish }), RELEASE),
    ];
    for (wire, op) in cases {
        let (bytes, end) = wire_bytes(&wire);
        assert_eq!(bytes[0], op);
        assert_eq!(Wire::take(&bytes[..end]), Some(wire));
        let step = match wire {
            Wire::Publish(step) | Wire::Invalidate(step) | Wire::Quiesce(step) | Wire::Release(step) => {
                Some(step)
            }
            _ => None,
        };
        for other in [ACQUIRE, PUBLISH, INVALIDATE, QUIESCE, RELEASE, ESTABLISH] {
            if other != op {
                let mut candidate = bytes;
                candidate[0] = other;
                let expected = if [PUBLISH, INVALIDATE, QUIESCE, RELEASE].contains(&other) {
                    step.map(|step| {
                        let frame = Step { op: other, ..step };
                        match other {
                            PUBLISH => Wire::Publish(frame),
                            INVALIDATE => Wire::Invalidate(frame),
                            QUIESCE => Wire::Quiesce(frame),
                            RELEASE => Wire::Release(frame),
                            _ => unreachable!(),
                        }
                    })
                } else {
                    None
                };
                assert_eq!(
                    Wire::take(&candidate[..end]),
                    expected,
                    "opcode {other} against a {op} body"
                );
            }
        }
        for unknown in [0, 7, 255] {
            let mut candidate = bytes;
            candidate[0] = unknown;
            assert_eq!(Wire::take(&candidate[..end]), None);
        }
    }
    assert_eq!(Wire::take(&[]), None);
    assert_eq!(Wire::take(&[0u8; Wire::LEN]), None);
}
