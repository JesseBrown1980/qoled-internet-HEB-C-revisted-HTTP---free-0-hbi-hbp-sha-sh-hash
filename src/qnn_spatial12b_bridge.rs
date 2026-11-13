//! Read-only native HTTP-0 address bridge for QNN spatial12b.
//! This module translates virtual coordinates. It does not load QNN weights,
//! run a neural network, perform any D3Q27 physics or populate 12B rooms.
#![forbid(unsafe_code)]

pub const CAPACITY: u64 = 12_000_000_000;
pub const CHANNELS: u64 = 27;
pub const WIDTH: u64 = 1_024;
pub const PLANE: u64 = WIDTH * WIDTH;
pub const QNN_SOURCE_COMMIT: &str = "6981cf54b1f289dabedac286108f4a5d4fa7f2f6";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Address {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub channel: u8,
}

#[must_use]
pub fn decode(id: u64) -> Option<Address> {
    if id >= CAPACITY {
        return None;
    }
    let room = id / CHANNELS;
    Some(Address {
        x: (room % WIDTH) as u16,
        y: ((room / WIDTH) % WIDTH) as u16,
        z: (room / PLANE) as u16,
        channel: (id % CHANNELS) as u8,
    })
}

#[must_use]
pub fn encode(a: Address) -> Option<u64> {
    if u64::from(a.x) >= WIDTH
        || u64::from(a.y) >= WIDTH
        || u64::from(a.channel) >= CHANNELS
    {
        return None;
    }
    let room = u64::from(a.z)
        .checked_mul(PLANE)?
        .checked_add(u64::from(a.y) * WIDTH)?
        .checked_add(u64::from(a.x))?;
    let id = room
        .checked_mul(CHANNELS)?
        .checked_add(u64::from(a.channel))?;
    (id < CAPACITY).then_some(id)
}

#[must_use]
pub fn path22(id: u64) -> Option<String> {
    if id >= CAPACITY {
        return None;
    }
    let mut value = id;
    let mut digits = [b'0'; 22];
    for digit in digits.iter_mut().rev() {
        // The final display is most-significant trit first.
        *digit = b'0' + (value % 3) as u8;
        value /= 3;
    }
    (value == 0).then(|| String::from_utf8(digits.to_vec()).expect("ASCII trits"))
}

#[must_use]
pub fn contract() -> String {
    format!(
        "HTTP0_QNN_SPATIAL12B_CONTRACT|capacity={CAPACITY}|channels_per_room={CHANNELS}|x_width={WIDTH}|address_trits=22|source_repo=JesseBrown1980/GGUF-files-operating-as-trained-quantum-neural-network-models|source_commit={QNN_SOURCE_COMMIT}|route=READ_ONLY|weights=NOT_LOADED|occupied_nodes=UNKNOWN|physics_engine=UNCHANGED|json=0\n"
    )
}

#[must_use]
pub fn lookup(id: u64) -> Option<String> {
    let a = decode(id)?;
    let ternary = path22(id)?;
    Some(format!(
        "HTTP0_QNN_SPATIAL12B_ADDRESS|id={id}|x={}|y={}|z={}|channel={}|ternary22={ternary}|state=UNAVAILABLE|model=NOT_LOADED|status=ADDRESS_ONLY|json=0\n",
        a.x, a.y, a.z, a.channel
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qnn_address_roundtrip_edges() {
        for id in [0, 1, 26, 27, 23_887_871, 23_887_872, CAPACITY - 1] {
            let a = decode(id).unwrap();
            assert_eq!(encode(a), Some(id));
            assert_eq!(path22(id).unwrap().len(), 22);
        }
        assert!(decode(CAPACITY).is_none());
        assert!(lookup(CAPACITY).is_none());
        assert!(encode(Address { x: 0, y: 0, z: 424, channel: 0 }).is_none());
    }

    #[test]
    fn cross_repo_qnn_last_address_fixture() {
        let id = CAPACITY - 1;
        let a = decode(id).unwrap();
        assert_eq!((a.x, a.y, a.z, a.channel), (796, 875, 423, 11));
        // This exact string is the known QNN Rust 1.81 hosted result.
        assert_eq!(path22(id).as_deref(), Some("1010222022010011100102"));
        assert!(lookup(id).unwrap().contains("|status=ADDRESS_ONLY|"));
        assert!(contract().contains("physics_engine=UNCHANGED"));
    }
}
