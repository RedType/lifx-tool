use anyhow::{Error, Result};
use std::convert::TryInto;
use crate::error::LifxError;

mod types;
pub use self::types::*;

const OTAP_TRUE:  u16 = 0b00_1_1_010000000000u16;
const OTAP_FALSE: u16 = 0b00_0_1_010000000000u16;
const RES_REQ: u8 = 0b01;
const ACK_REQ: u8 = 0b10;

macro_rules! reserved {
    ($pack:ident, $bytes:expr) => {
        $pack.extend_from_slice(&[0u8; $bytes]);
    };
}

pub fn pack(header: Header, msg: Message) -> Vec<u8> {
    let mut pack = Vec::new();
    reserved!(pack, 2); // size
    let otap = if header.tagged { OTAP_TRUE } else { OTAP_FALSE };
    pack.extend_from_slice(&otap.to_le_bytes());
    pack.extend_from_slice(&header.source.to_le_bytes());
    pack.extend_from_slice(&header.target.to_le_bytes());
    reserved!(pack, 6);
    pack.push(if header.res_required { RES_REQ } else { 0u8 }
            | if header.ack_required { ACK_REQ } else { 0u8 });
    pack.push(header.sequence);
    reserved!(pack, 8);
    pack.extend_from_slice(&msg.id().to_le_bytes());
    reserved!(pack, 2);

    match msg {
        Message::GetService => (),
        Message::StateService { service, port } => {
            pack.push(service);
            pack.extend_from_slice(&port.to_le_bytes());
        },
        Message::Get => (),
        Message::SetColor { color, duration } => {
            reserved!(pack, 1);
            pack.extend_from_slice(&color.pack());
            pack.extend_from_slice(&duration.to_le_bytes());
        },
        Message::State { color, power, label } => {
            pack.extend_from_slice(&color.pack());
            reserved!(pack, 2);
            pack.extend_from_slice(&power.to_le_bytes());
            pack.extend_from_slice(&label.0);
            reserved!(pack, 8);
        },
    }

    // set packet size
    let sz = (pack.len() as u16).to_le_bytes();
    pack[0] = sz[0];
    pack[1] = sz[1];

    pack
}

pub fn unpack(pack: &[u8]) -> Result<(Header, Message)> {
    let len = pack.len();
    if len < Header::SIZE {
        return Err(Error::new(LifxError::WrongSize {
            found: len,
            expected: Header::SIZE,
        }));
    }
    let id = u16::from_le_bytes(pack[32..=33].try_into()?);
    if len < Header::SIZE + id_to_size(id) {
        return Err(Error::new(LifxError::WrongSize {
            found: len,
            expected: Header::SIZE + id_to_size(id),
        }));
    }

    let header = Header {
        tagged: match u16::from_le_bytes(pack[2..=3].try_into()?) {
            OTAP_TRUE => true,
            OTAP_FALSE => false,
            _ => return Err(Error::new(LifxError::MalformedHeader)),
        },
        source: u32::from_le_bytes(pack[4..=7].try_into()?),
        target: u64::from_le_bytes(pack[8..=15].try_into()?),
        res_required: pack[22] & RES_REQ != 0,
        ack_required: pack[22] & ACK_REQ != 0,
        sequence: pack[23],
    };

    let msg_pack = &pack[Header::SIZE..];

    let message = match id {
        message_type::GET_SERVICE => Message::GetService,
        message_type::STATE_SERVICE => Message::StateService {
            service: msg_pack[0],
            port: u32::from_le_bytes(msg_pack[1..=4].try_into()?),
        },
        message_type::GET => Message::Get,
        message_type::SET_COLOR => Message::SetColor {
            color: HSBK::unpack(&msg_pack[1..=8])?,
            duration: u32::from_le_bytes(msg_pack[9..=12].try_into()?),
        },
        message_type::STATE => Message::State {
            color: HSBK::unpack(&msg_pack[0..=7])?,
            power: u16::from_le_bytes(msg_pack[10..=11].try_into()?),
            label: LifxString::from_buf(&msg_pack[12..=15])?,
        },
        x => return Err(Error::new(LifxError::UnknownMessageType(x))),
    };

    Ok((header, message))
}

macro_rules! message_types {
    { $($enum_n:ident $const_n:ident = $val:literal $sz:literal),+ $(,)? } => {
        pub mod message_type {
            $(pub const $const_n: u16 = $val;)+
        }

        fn id_to_size(id: u16) -> usize {
            match id {
                $($val => $sz,)+
                _ => 0,
            }
        }

        impl Message {
            fn id(&self) -> u16 {
                match self {
                    $(Message::$enum_n {..} => $val),+
                }
            }
        }
    };
}

// enum name, const name, id, size in bytes
message_types! {
    GetService GET_SERVICE = 2 0,
    StateService STATE_SERVICE = 3 5,
    Get GET = 101 0,
    SetColor SET_COLOR = 102 13,
    State STATE = 107 24,
}

#[test]
fn packet_matches_example() {
    let example = [
        0x31u8, 0x00u8, 0x00u8, 0x34u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x66u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x55u8, 0x55u8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xACu8, 0x0Du8,
        0x00u8, 0x04u8, 0x00u8, 0x00u8,
    ];

    let my_packet = pack(Header {
        tagged: true,
        .. Default::default()
    }, Message::SetColor {
        color: HSBK::from_hsvk(120.0, 1.0, 1.0, 3500),
        duration: 1024,
    });

    let res = example.iter().zip(my_packet.iter()).enumerate()
                .map(|(n, (a, b))| (n, a, b))
                .filter(|(_, a, b)| a != b)
                .collect::<Vec<_>>();

    if !res.is_empty() {
        panic!("Mismatched bytes: {:?}", res);
    }
}
