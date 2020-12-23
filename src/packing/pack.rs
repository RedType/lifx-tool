use num_traits::*;

use super::messages::{*, Message::*};
use super::types::*;

fn label_helper(label: &str) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    let size = std::cmp::min(label.len(), 32);
    for i in 0..size {
        buffer[i] = label.as_bytes()[i];
    }
    buffer
}

pub struct PacketBuilder(Packet);
pub struct Header(Packet);

impl PacketBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn header(
        mut self,
        tagged: bool,
        source: u32,
        target: MACAddress,
        res_required: bool,
        ack_required: bool,
        sequence: u8,
    ) -> Header {
        self.0.extend_from_slice(&[0u8; 2]); // size (to be filled later)
        self.0.push(0u8); // low byte of origin, tagged, addressable, protocol
        if tagged {
            self.0.push(0b00_1_1_0100);
        } else {
            self.0.push(0b00_0_1_0100);
        }
        self.0.extend_from_slice(&source.to_le_bytes());
        match target {
            MACAddress::All => self.0.extend_from_slice(&[0u8; 8]),
            MACAddress::Eui48(addr) => {
                self.0.extend_from_slice(&addr);
                self.0.extend_from_slice(&[0u8; 2]);
            },
            MACAddress::Eui64(_addr) =>
                unimplemented!("LIFX devices don't accept EUI64 addresses"),
        }
        self.0.extend_from_slice(&[0u8; 6]); // reserved
        self.0.push(
            if res_required { 0b01u8 } else { 0u8 } |
            if ack_required { 0b10u8 } else { 0u8 }
        );
        self.0.push(sequence);
        self.0.extend_from_slice(&[0u8; 12]); // 8 reserved,
                                              // 2 type (to be filled later),
                                              // 2 reserved

        Header(self.0)
    }
}

impl Header {

    // set the size and return the finished packet
    fn finalize(&mut self) -> Packet {
        let val = (self.0.len() as u16).to_le_bytes();
        for i in 0..2 {
            self.0[i] = val[i];
        }
        self.0.to_owned()
    }

    fn set_type(&mut self, val: Message) {
        let val = val.to_u16().unwrap().to_le_bytes();
        let offset = 32;
        for i in 0..2 {
            self.0[i + offset] = val[i + offset];
        }
    }

    // np for no payload
    fn np(mut self, val: Message) -> Packet {
        self.set_type(val);
        self.finalize()
    }

    // Device Messages
    pub fn get_service(mut self) -> Packet { self.np(GetService) }

    pub fn state_service(mut self, service: u8, port: u32) -> Packet {
        self.set_type(StateService);
        self.0.push(service);
        self.0.extend_from_slice(&port.to_le_bytes());
        self.finalize()
    }

    pub fn get_host_info(mut self) -> Packet { self.np(GetHostInfo) }

    pub fn state_host_info(mut self, signal: f32, tx: u32, rx: u32) -> Packet {
        self.set_type(StateHostInfo);
        self.0.extend_from_slice(&signal.to_le_bytes());
        self.0.extend_from_slice(&tx.to_le_bytes());
        self.0.extend_from_slice(&rx.to_le_bytes());
        self.0.extend_from_slice(&[0u8; 2]); // reserved
        self.finalize()
    }

    pub fn get_host_firmware(mut self) -> Packet { self.np(GetHostFirmware) }

    pub fn state_host_firmware(
        mut self,
        build: u64,
        version_minor: u16,
        version_major: u16,
    ) -> Packet {
        self.set_type(StateHostFirmware);
        self.0.extend_from_slice(&build.to_le_bytes());
        self.0.extend_from_slice(&[0u8; 8]); // reserved
        self.0.extend_from_slice(&version_minor.to_le_bytes());
        self.0.extend_from_slice(&version_major.to_le_bytes());
        self.finalize()
    }

    pub fn get_wifi_info(mut self) -> Packet { self.np(GetWifiInfo) }

    pub fn state_wifi_info(mut self, signal: f32, tx: u32, rx: u32) -> Packet {
        self.set_type(StateHostInfo);
        self.0.extend_from_slice(&signal.to_le_bytes());
        self.0.extend_from_slice(&tx.to_le_bytes());
        self.0.extend_from_slice(&rx.to_le_bytes());
        self.0.extend_from_slice(&[0u8; 2]); // reserved
        self.finalize()
    }

    pub fn get_wifi_firmware(mut self) -> Packet { self.np(GetWifiFirmware) }

    pub fn state_wifi_firmware(
        mut self,
        build: u64,
        version_minor: u16,
        version_major: u16,
    ) -> Packet {
        self.set_type(StateHostFirmware);
        self.0.extend_from_slice(&build.to_le_bytes());
        self.0.extend_from_slice(&[0u8; 8]); // reserved
        self.0.extend_from_slice(&version_minor.to_le_bytes());
        self.0.extend_from_slice(&version_major.to_le_bytes());
        self.finalize()
    }

    pub fn get_power(mut self) -> Packet { self.np(GetPower) }

    pub fn set_power(mut self, powered: bool) -> Packet {
        self.set_type(SetPower);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.finalize()
    }

    pub fn state_power(mut self, powered: bool) -> Packet {
        self.set_type(StatePower);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.finalize()
    }

    pub fn get_label(mut self) -> Packet { self.np(GetLabel) }

    pub fn set_label(mut self, label: &str) -> Packet {
        self.set_type(SetLabel);
        self.0.extend_from_slice(&label_helper(label));
        self.finalize()
    }

    pub fn state_label(mut self, label: &str) -> Packet {
        self.set_type(StateLabel);
        self.0.extend_from_slice(&label_helper(label));
        self.finalize()
    }

    pub fn get_version(mut self) -> Packet { self.np(GetVersion) }

    pub fn state_version(
        mut self,
        vendor: u32,
        product: u32,
        version: u32,
    ) -> Packet {
        self.set_type(StateVersion);
        self.0.extend_from_slice(&vendor.to_le_bytes());
        self.0.extend_from_slice(&product.to_le_bytes());
        self.0.extend_from_slice(&version.to_le_bytes());
        self.finalize()
    }

    pub fn get_info(mut self) -> Packet { self.np(GetInfo) }

    pub fn state_info(
        mut self,
        time: u64,
        uptime: u64,
        downtime: u64,
    ) -> Packet {
        self.set_type(StateInfo);
        self.0.extend_from_slice(&time.to_le_bytes());
        self.0.extend_from_slice(&uptime.to_le_bytes());
        self.0.extend_from_slice(&downtime.to_le_bytes());
        self.finalize()
    }

    pub fn acknowledgement(mut self) -> Packet { self.np(Acknowledgement) }

    pub fn get_location(mut self) -> Packet { self.np(GetLocation) }

    pub fn set_location(
        mut self,
        location: &[u8; 16],
        label: &str,
        updated_at: i64,
    ) -> Packet {
        self.set_type(SetLocation);
        self.0.extend_from_slice(location);
        self.0.extend_from_slice(&label_helper(label));
        self.0.extend_from_slice(&updated_at.to_le_bytes());
        self.finalize()
    }

    pub fn state_location(
        mut self,
        location: &[u8; 16],
        label: &str,
        updated_at: i64,
    ) -> Packet {
        self.set_type(StateLocation);
        self.0.extend_from_slice(location);
        self.0.extend_from_slice(&label_helper(label));
        self.0.extend_from_slice(&updated_at.to_le_bytes());
        self.finalize()
    }

    pub fn get_group(mut self) -> Packet { self.np(GetGroup) }

    pub fn set_group(
        mut self,
        group: &[u8; 16],
        label: &str,
        updated_at: i64,
    ) -> Packet {
        self.set_type(SetGroup);
        self.0.extend_from_slice(group);
        self.0.extend_from_slice(&label_helper(label));
        self.0.extend_from_slice(&updated_at.to_le_bytes());
        self.finalize()
    }

    pub fn state_group(
        mut self,
        group: &[u8; 16],
        label: &str,
        updated_at: i64,
    ) -> Packet {
        self.set_type(StateGroup);
        self.0.extend_from_slice(group);
        self.0.extend_from_slice(&label_helper(label));
        self.0.extend_from_slice(&updated_at.to_le_bytes());
        self.finalize()
    }

    pub fn echo_request(mut self, payload: &[u8; 8]) -> Packet {
        self.set_type(EchoRequest);
        self.0.extend_from_slice(payload);
        self.finalize()
    }

    pub fn echo_response(mut self, payload: &[u8; 8]) -> Packet {
        self.set_type(EchoRequest);
        self.0.extend_from_slice(payload);
        self.finalize()
    }
}
