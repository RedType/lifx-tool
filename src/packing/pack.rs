use num_traits::*;

use super::messages::{*, Message::*};
use super::types::*;
use super::util;

#[derive(Clone)]
pub struct PacketBuilder(Packet);
#[derive(Clone)]
pub struct Header(Packet);

impl PacketBuilder {
    fn reserve(&mut self, count_bytes: usize) {
        self.0.extend_from_slice(&vec![0u8; count_bytes]);
    }

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
                self.reserve(2);
            },
            MACAddress::Eui64(_addr) =>
                unimplemented!("LIFX devices don't accept EUI64 addresses"),
        }
        self.reserve(6);
        self.0.push(
            if res_required { 0b01u8 } else { 0u8 } |
            if ack_required { 0b10u8 } else { 0u8 }
        );
        self.0.push(sequence);
        self.reserve(12); // 8 reserved
                          // 2 type (to be filled later)
                          // 2 reserved

        Header(self.0)
    }
}

impl Header {
    // Private Helpers

    // set the size and return the finished packet
    fn finalize(&mut self) -> Packet {
        // make sure that type is set
        if self.0[32] == 0 && self.0[33] == 0 {
            panic!("Packet type hasn't been set (this is a bug)");
        }

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
            self.0[i + offset] = val[i];
        }
    }

    // no payload
    fn np(mut self, val: Message) -> Packet {
        self.set_type(val);
        self.finalize()
    }

    fn reserve(&mut self, count_bytes: usize) {
        self.0.extend_from_slice(&vec![0u8; count_bytes]);
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
        self.reserve(2);
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
        self.reserve(8);
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
        self.reserve(2);
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
        self.reserve(8);
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
        self.0.extend_from_slice(&util::label_helper(label));
        self.finalize()
    }

    pub fn state_label(mut self, label: &str) -> Packet {
        self.set_type(StateLabel);
        self.0.extend_from_slice(&util::label_helper(label));
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
        self.0.extend_from_slice(&util::label_helper(label));
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
        self.0.extend_from_slice(&util::label_helper(label));
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
        self.0.extend_from_slice(&util::label_helper(label));
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
        self.0.extend_from_slice(&util::label_helper(label));
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

    // Light Messages

    pub fn get(mut self) -> Packet { self.np(Get) }

    pub fn set_color(mut self, color: HSBK, duration: u32) -> Packet {
        self.set_type(SetColor);
        self.reserve(1);
        self.0.extend_from_slice(&color.pack());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.finalize()
    }

    pub fn set_waveform(
        mut self,
        transient: bool,
        color: HSBK,
        period: u32,
        cycles: f32,
        skew_ratio: f32,
        waveform: u8,
    ) -> Packet {
        // transform from [0, 1] to [-32768, 32767] per spec
        let skew =
            (util::clamp(skew_ratio, 0.0, 1.0) * 65535.0 - 32768.0) as u16;

        self.set_type(SetWaveform);
        self.reserve(1);
        self.0.push(if transient { 1u8 } else { 0u8 });
        self.0.extend_from_slice(&color.pack());
        self.0.extend_from_slice(&period.to_le_bytes());
        self.0.extend_from_slice(&cycles.to_le_bytes());
        self.0.extend_from_slice(&skew.to_le_bytes());
        self.0.push(waveform);
        self.finalize()
    }

    pub fn set_waveform_optional(
        mut self,
        transient: bool,
        color: HSBK,
        period: u32,
        cycles: f32,
        skew_ratio: f32,
        waveform: u8,
        set_hue: bool,
        set_saturation: bool,
        set_brightness: bool,
        set_kelvin: bool,
    ) -> Packet {
        // transform from [0, 1] to [-32768, 32767] per spec
        let skew =
            (util::clamp(skew_ratio, 0.0, 1.0) * 65535.0 - 32768.0) as u16;

        self.set_type(SetWaveformOptional);
        self.reserve(1);
        self.0.push(if transient { 1u8 } else { 0u8 });
        self.0.extend_from_slice(&color.pack());
        self.0.extend_from_slice(&period.to_le_bytes());
        self.0.extend_from_slice(&cycles.to_le_bytes());
        self.0.extend_from_slice(&skew_ratio.to_le_bytes());
        self.0.push(waveform);
        self.0.push(if set_hue { 1u8 } else { 0u8 });
        self.0.push(if set_saturation { 1u8 } else { 0u8 });
        self.0.push(if set_brightness { 1u8 } else { 0u8 });
        self.0.push(if set_kelvin { 1u8 } else { 0u8 });
        self.finalize()
    }

    pub fn state(
        mut self,
        color: HSBK,
        powered: bool,
        label: &str,
    ) -> Packet {
        self.set_type(State);
        self.0.extend_from_slice(&color.pack());
        self.reserve(2);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.0.extend_from_slice(&util::label_helper(label));
        self.reserve(8);
        self.finalize()
    }

    pub fn get_light_power(mut self) -> Packet { self.np(GetLightPower) }

    pub fn set_light_power(mut self, powered: bool, duration: u32) -> Packet {
        self.set_type(SetLightPower);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.finalize()
    }

    pub fn state_light_power(mut self, powered: bool) -> Packet {
        self.set_type(SetLightPower);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.finalize()
    }

    pub fn get_infrared(mut self) -> Packet { self.np(GetInfrared) }

    pub fn state_infrared(mut self, brightness: u16) -> Packet {
        self.set_type(StateInfrared);
        self.0.extend_from_slice(&brightness.to_le_bytes());
        self.finalize()
    }

    pub fn set_infrared(mut self, brightness: u16) -> Packet {
        self.set_type(SetInfrared);
        self.0.extend_from_slice(&brightness.to_le_bytes());
        self.finalize()
    }

    // MultiZone Messages

    pub fn set_extended_color_zones(
        mut self,
        duration: u32,
        apply: ApplicationRequest,
        index: u16,
        colors_count: u8,
        colors: &[HSBK; 82],
    ) -> Packet {
        self.set_type(SetExtendedColorZones);
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.0.push(apply as u8);
        self.0.extend_from_slice(&index.to_le_bytes());
        self.0.push(colors_count);
        for color in colors {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    pub fn get_extended_color_zones(mut self) -> Packet {
        self.np(GetExtendedColorZones)
    }

    pub fn state_extended_color_zones(
        mut self,
        count: u16,
        index: u16,
        colors_count: u8,
        colors: &[HSBK; 82],
    ) -> Packet {
        self.set_type(StateExtendedColorZones);
        self.0.extend_from_slice(&count.to_le_bytes());
        self.0.extend_from_slice(&index.to_le_bytes());
        self.0.push(colors_count);
        for color in colors {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    pub fn set_color_zones(
        mut self,
        start_index: u8,
        end_index: u8,
        color: HSBK,
        duration: u32,
        apply: ApplicationRequest,
    ) -> Packet {
        self.set_type(SetColorZones);
        self.0.push(start_index);
        self.0.push(end_index);
        self.0.extend_from_slice(&color.pack());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.0.push(apply as u8);
        self.finalize()
    }

    pub fn get_color_zones(
        mut self,
        start_index: u8,
        end_index: u8,
    ) -> Packet {
        self.set_type(GetColorZones);
        self.0.push(start_index);
        self.0.push(end_index);
        self.finalize()
    }

    pub fn state_zone(
        mut self,
        count: u8,
        index: u8,
        color: HSBK,
    ) -> Packet {
        self.set_type(StateZone);
        self.0.push(count);
        self.0.push(index);
        self.0.extend_from_slice(&color.pack());
        self.finalize()
    }

    pub fn state_multi_zone(
        mut self,
        count: u8,
        index: u8,
        colors: &[HSBK; 8],
    ) -> Packet {
        self.set_type(StateMultiZone);
        self.0.push(count);
        self.0.push(index);
        for color in colors {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    // Tile Messages

    pub fn get_device_chain(mut self) -> Packet { self.np(GetDeviceChain) }

    pub fn state_device_chain(
        mut self,
        start_index: u8,
        tile_devices: &[Tile; 16],
        total_count: u8,
    ) -> Packet {
        self.set_type(StateDeviceChain);
        self.0.push(start_index);
        for tile in tile_devices {
            self.0.extend(tile.pack());
        }
        self.0.push(total_count);
        self.finalize()
    }

    pub fn set_user_position(
        mut self,
        tile_index: u8,
        user_x: f32,
        user_y: f32,
    ) -> Packet {
        self.set_type(SetUserPosition);
        self.0.push(tile_index);
        self.reserve(2);
        self.0.extend_from_slice(&user_x.to_le_bytes());
        self.0.extend_from_slice(&user_y.to_le_bytes());
        self.finalize()
    }

    pub fn get_tile_state_64(
        mut self,
        tile_index: u8,
        length: u8,
        x: u8,
        y: u8,
        width: u8,
    ) -> Packet {
        self.set_type(GetTileState64);
        self.0.push(tile_index);
        self.0.push(length);
        self.reserve(1);
        self.0.push(x);
        self.0.push(y);
        self.0.push(width);
        self.finalize()
    }

    pub fn state_tile_state_64(
        mut self,
        tile_index: u8,
        x: u8,
        y: u8,
        width: u8,
        colors: &[HSBK; 64],
    ) -> Packet {
        self.set_type(GetTileState64);
        self.0.push(tile_index);
        self.reserve(1);
        self.0.push(x);
        self.0.push(y);
        self.0.push(width);
        for color in colors {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    pub fn set_tile_state_64(
        mut self,
        tile_index: u8,
        length: u8,
        x: u8,
        y: u8,
        width: u8,
        duration: u32,
        colors: &[HSBK; 64],
    ) -> Packet {
        self.set_type(GetTileState64);
        self.0.push(tile_index);
        self.0.push(length);
        self.reserve(1);
        self.0.push(x);
        self.0.push(y);
        self.0.push(width);
        self.0.extend_from_slice(&duration.to_le_bytes());
        for color in colors {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    // Switch Messages

    pub fn get_relay_power(mut self, relay_index: u8) -> Packet {
        self.set_type(GetRelayPower);
        self.0.push(relay_index);
        self.finalize()
    }

    pub fn set_relay_power(
        mut self,
        relay_index: u8,
        powered: bool,
    ) -> Packet {
        self.set_type(SetRelayPower);
        self.0.push(relay_index);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.finalize()
    }

    pub fn state_relay_power(
        mut self,
        relay_index: u8,
        powered: bool,
    ) -> Packet {
        self.set_type(StateRelayPower);
        self.0.push(relay_index);
        self.0.extend_from_slice(
            &(if powered { 65535u16 } else { 0u16 }).to_le_bytes());
        self.finalize()
    }

    // Firmware Effects

    pub fn set_multi_zone_effect(
        mut self,
        instanceid: u32,
        etype: MultiZoneEffectType,
        speed: u32,
        duration: u64,
        parameters: &[u32; 8],
    ) -> Packet {
        self.set_type(SetMultiZoneEffect);
        self.0.extend_from_slice(&instanceid.to_le_bytes());
        self.0.push(etype as u8);
        self.reserve(2);
        self.0.extend_from_slice(&speed.to_le_bytes());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.reserve(8);
        for parameter in parameters {
            self.0.extend_from_slice(&parameter.to_le_bytes());
        }
        self.finalize()
    }

    pub fn set_tile_effect(
        mut self,
        instanceid: u32,
        etype: TileEffectType,
        speed: u32,
        duration: u64,
        parameters: &[u32; 8],
        palette_count: u8,
        palette: &[HSBK; 16],
    ) -> Packet {
        self.set_type(SetTileEffect);
        self.reserve(2);
        self.0.extend_from_slice(&instanceid.to_le_bytes());
        self.0.push(etype as u8);
        self.0.extend_from_slice(&speed.to_le_bytes());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.reserve(8);
        for parameter in parameters {
            self.0.extend_from_slice(&parameter.to_le_bytes());
        }
        self.0.push(palette_count);
        for color in palette {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }

    pub fn get_tile_effect(mut self) -> Packet { self.np(GetTileEffect) }

    pub fn state_tile_effect(
        mut self,
        instanceid: u32,
        etype: TileEffectType,
        speed: u32,
        duration: u64,
        parameters: &[u32; 8],
        palette_count: u8,
        palette: &[HSBK; 16],
    ) -> Packet {
        self.set_type(SetTileEffect);
        self.reserve(2);
        self.0.extend_from_slice(&instanceid.to_le_bytes());
        self.0.push(etype as u8);
        self.0.extend_from_slice(&speed.to_le_bytes());
        self.0.extend_from_slice(&duration.to_le_bytes());
        self.reserve(8);
        for parameter in parameters {
            self.0.extend_from_slice(&parameter.to_le_bytes());
        }
        self.0.push(palette_count);
        for color in palette {
            self.0.extend_from_slice(&color.pack());
        }
        self.finalize()
    }
}

// Tests

#[test]
fn test_all_packets_build() {
    let header = PacketBuilder::new().header(
        true,
        0,
        MACAddress::Eui48([0, 0, 0, 0, 0, 0]),
        false,
        false,
        0,
    );

    header.clone().get_service();
    header.clone().state_service(0, 0);
    header.clone().get_host_info();
    header.clone().state_host_info(0.0, 0, 0);
    header.clone().get_host_firmware();
    header.clone().state_host_firmware(0, 0, 0);
    header.clone().get_wifi_info();
    header.clone().state_wifi_info(0.0, 0, 0);
    header.clone().get_wifi_firmware();
    header.clone().state_wifi_firmware(0, 0, 0);
    header.clone().get_power();
    header.clone().set_power(true);
    header.clone().state_power(true);
    header.clone().get_label();
    header.clone().set_label("");
    header.clone().state_label("");
    header.clone().get_version();
    header.clone().state_version(0, 0, 0);
    header.clone().get_info();
    header.clone().state_info(0, 0, 0);
    header.clone().acknowledgement();
    header.clone().get_location();
    header.clone().set_location(&[0u8; 16], "", 0);
    header.clone().state_location(&[0u8; 16], "", 0);
    header.clone().get_group();
    header.clone().set_group(&[0u8; 16], "", 0);
    header.clone().state_group(&[0u8; 16], "", 0);
    header.clone().echo_request(&[0u8; 8]);
    header.clone().echo_response(&[0u8; 8]);
    header.clone().get();
    header.clone().set_waveform(
        true,
        HSBK::new(0.0, 0, 0, 5000),
        0, 0.0, 0.0, 0,
    );
    header.clone().set_waveform_optional(
        true,
        HSBK::new(0.0, 0, 0, 5000),
        0, 0.0, 0.0, 0,
        true, true, true, true,
    );
    header.clone().state(HSBK::new(0.0, 0, 0, 5000), true, "");
    header.clone().get_light_power();
    header.clone().set_light_power(true, 0);
    header.clone().state_light_power(true);
    header.clone().get_infrared();
    header.clone().state_infrared(0);
    header.clone().set_infrared(0);
    header.clone().set_extended_color_zones(
        0, ApplicationRequest::Apply, 0, 0,
        &[HSBK::new(0.0, 0, 0, 5000); 82],
    );
    header.clone().get_extended_color_zones();
    header.clone().state_extended_color_zones(
        0, 0, 0,
        &[HSBK::new(0.0, 0, 0, 5000); 82],
    );
    header.clone().set_color_zones(
        0, 0, HSBK::new(0.0, 0, 0, 5000), 0,
        ApplicationRequest::ApplyOnly,
    );
    header.clone().get_color_zones(0, 0);
    header.clone().state_zone(0, 0, HSBK::new(0.0, 0, 0, 5000));
    header.clone().state_multi_zone(0, 0, &[HSBK::new(0.0, 0, 0, 5000); 8]);
    header.clone().get_device_chain();
    header.clone().state_device_chain(
        0,
        &[Tile::new(0, 0, 0, 0.0, 0.0, 0, 0, 0, 0, 0, 0, 0, 0); 16],
        0,
    );
    header.clone().set_user_position(0, 0.0, 0.0);
    header.clone().get_tile_state_64(0, 0, 0, 0, 0);
    header.clone().state_tile_state_64(
        0, 0, 0, 0,
        &[HSBK::new(0.0, 0, 0, 5000); 64]
    );
    header.clone().set_tile_state_64(
        0, 0, 0, 0, 0, 0,
        &[HSBK::new(0.0, 0, 0, 5000); 64]
    );
    header.clone().get_relay_power(0);
    header.clone().set_relay_power(0, true);
    header.clone().state_relay_power(0, true);
    header.clone().set_multi_zone_effect(
        0, MultiZoneEffectType::Move, 0, 0, &[0u32; 8]);
    header.clone().set_tile_effect(
        0, TileEffectType::Flame, 0, 0, &[0u32; 8], 0,
        &[HSBK::new(0.0, 0, 0, 5000); 16]);
    header.clone().get_tile_effect();
    header.clone().state_tile_effect(
        0, TileEffectType::Flame, 0, 0, &[0u32; 8], 0,
        &[HSBK::new(0.0, 0, 0, 5000); 16]);
}

#[test]
pub fn test_packet_matches_example() {
    // example taken from protocol documentation section Building a LIFX Packet
    let example = vec![
        0x31u8, 0x00u8, 0x00u8, 0x34u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x66u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x55u8, 0x55u8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xACu8, 0x0Du8,
        0x00u8, 0x04u8, 0x00u8, 0x00u8,
    ];

    let test = PacketBuilder::new()
        .header(true, 0, MACAddress::All, false, false, 0)
        .set_color(HSBK::new(120.0, u16::MAX, u16::MAX, 3500), 1024);

    let result = example.iter().zip(test.iter())
        .filter(|(a, b)| a != b)
        .enumerate()
        .collect::<Vec<_>>();

    if !result.is_empty() {
        panic!("Mismatched bytes: {:?}", result);
    }
}
