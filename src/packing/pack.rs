use super::{
    types::*,
    util,
};

const TYPE_OFFSET: usize = 32;

pub fn pack(header: &Header, message: &dyn Message) -> Vec<u8> {
    let mut pack = header.pack();
    pack.extend_from_slice(&message.pack());
    let msg_type = (message.get_type() as u16).to_le_bytes();
    let pack_size = (pack.len() as u16).to_le_bytes();
    for i in 0..2 {
        pack[i] = pack_size[i];
        pack[i + TYPE_OFFSET] = msg_type[i];
    }
    pack
}

macro_rules! primitive_packs {
    ($($t:ty),+ $(,)?) => {
        $(impl Pack for $t {
            fn pack(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
        impl Pack for &$t {
            fn pack(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        })+
    };
}

primitive_packs!(u8, u16, u32, u64, i16, i64, f32);

struct PackHelper(Vec<u8>);

impl PackHelper {
    fn new() -> Self { PackHelper(Vec::new()) }

    fn push<T: Pack>(&mut self, v: T) -> &mut Self {
        self.0.extend_from_slice(&v.pack());
        self
    }

    fn push_all<I>(&mut self, vs: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Pack,
    {
        for v in vs {
            self.0.extend_from_slice(&v.pack());
        }
        self
    }

    fn push_slice(&mut self, slice: &[u8]) -> &mut Self {
        self.0.extend_from_slice(slice);
        self
    }

    fn reserve(&mut self, bytes: usize) -> &mut Self {
        self.0.reserve(bytes);
        for _ in 0..bytes {
            self.0.push(0);
        }
        self
    }

    fn push_bool(&mut self, b: bool) -> &mut Self {
        self.0.push(if b { 1u8 } else { 0u8 });
        self
    }

    fn push_powered(&mut self, p: bool) -> &mut Self {
        self.push(if p { 65535u16 } else { 0u16 });
        self
    }

    fn pack(&mut self) -> Vec<u8> { self.0.clone() }
}

impl Pack for MacAddress {
    fn pack(&self) -> Vec<u8> {
        let mut pack = PackHelper::new();
        match self {
            MacAddress::All => { pack.reserve(8); },
            MacAddress::Eui48(addr) => { pack.push_slice(addr).reserve(2); },
            //MacAddress::Eui64(_addr) =>
                //unimplemented!("LIFX devices don't accept EUI64 addresses"),
        }
        pack.pack()
    }
}

impl Pack for HSBK {
    fn pack(&self) -> Vec<u8> {
        // hue value is scaled from 0-360 to 0-65535
        let hue_conv = self.hue() / 360.0 * 65535.0;
        PackHelper::new()
            .push(hue_conv as u16)
            .push(self.saturation())
            .push(self.brightness())
            .push(self.kelvin())
            .pack()
    }
}

impl Pack for &HSBK {
    fn pack(&self) -> Vec<u8> {
        (*self).pack()
    }
}

impl Pack for Tile {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.accel_meas_x)
            .push(self.accel_meas_y)
            .push(self.accel_meas_z)
            .reserve(2)
            .push(self.user_x)
            .push(self.user_y)
            .push(self.width)
            .push(self.height)
            .reserve(1)
            .push(self.device_vendor)
            .push(self.device_product)
            .push(self.device_version)
            .push(self.firmware_build)
            .reserve(8)
            .push(self.firmware_minor)
            .push(self.firmware_major)
            .reserve(4)
            .pack()
    }
}

impl Pack for &Tile {
    fn pack(&self) -> Vec<u8> {
        (*self).pack()
    }
}

impl Pack for Header {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .reserve(3) // size (to be filled later) and low byte of origin,
                        // tagged, addressable, protocol
            .push(if self.tagged { 0b00_1_1_0100u8 } else { 0b00_0_1_0100u8 })
            .push(self.source)
            .push(self.target)
            .reserve(6)
            .push(
            if self.res_required { 0b01u8 } else { 0u8 } |
            if self.ack_required { 0b10u8 } else { 0u8 }
            )
            .push(self.sequence)
            .reserve(12) // 8 reserved
                         // 2 type (to be filled later)
                         // 2 reserved
            .pack()
    }
}

// Device Messages
impl Pack for GetService {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateService {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.service)
        .push(self.port)
        .pack()
    }
}

impl Pack for GetHostInfo {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateHostInfo {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.signal)
        .push(self.tx)
        .push(self.rx)
        .pack()
    }
}

impl Pack for GetHostFirmware {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateHostFirmware {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.build)
        .reserve(8)
        .push(self.version_minor)
        .push(self.version_major)
        .pack()
    }
}

impl Pack for GetWifiInfo {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateWifiInfo {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.signal)
        .push(self.tx)
        .push(self.rx)
        .reserve(2)
        .pack()
    }
}

impl Pack for GetWifiFirmware {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateWifiFirmware {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.build)
        .reserve(8)
        .push(self.version_minor)
        .push(self.version_major)
        .pack()
    }
}

impl Pack for GetPower {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_powered(self.powered)
        .pack()
    }
}

impl Pack for StatePower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_powered(self.powered)
        .pack()
    }
}

impl Pack for GetLabel {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetLabel {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.label)
        .pack()
    }
}

impl Pack for StateLabel {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.label)
        .pack()
    }
}

impl Pack for GetVersion {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateVersion {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.vendor)
        .push(self.product)
        .push(self.version)
        .pack()
    }
}

impl Pack for GetInfo {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateInfo {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.time)
        .push(self.uptime)
        .push(self.downtime)
        .pack()
    }
}

impl Pack for Acknowledgement {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for GetLocation {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetLocation {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.location)
        .push_slice(&self.label)
        .push(self.updated_at)
        .pack()
    }
}

impl Pack for StateLocation {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.location)
        .push_slice(&self.label)
        .push(self.updated_at)
        .pack()
    }
}

impl Pack for GetGroup {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetGroup {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.group)
        .push_slice(&self.label)
        .push(self.updated_at)
        .pack()
    }
}

impl Pack for StateGroup {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.group)
        .push_slice(&self.label)
        .push(self.updated_at)
        .pack()
    }
}

impl Pack for EchoRequest {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.payload)
        .pack()
    }
}

impl Pack for EchoResponse {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_slice(&self.payload)
        .pack()
    }
}


// Light Messages
impl Pack for Get {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetColor {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .reserve(1)
        .push(self.color)
        .push(self.duration)
        .pack()
    }
}

impl Pack for SetWaveform {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .reserve(1)
        .push_bool(self.transient)
        .push(self.color)
        .push(self.period)
        .push(self.cycles)
        .push(util::conv_skew(self.skew_ratio))
        .push(self.waveform)
        .pack()
    }
}

impl Pack for SetWaveformOptional {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .reserve(1)
        .push_bool(self.transient)
        .push(self.color)
        .push(self.period)
        .push(self.cycles)
        .push(util::conv_skew(self.skew_ratio))
        .push(self.waveform)
        .push_bool(self.set_hue)
        .push_bool(self.set_saturation)
        .push_bool(self.set_brightness)
        .push_bool(self.set_kelvin)
        .pack()
    }
}

impl Pack for State {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.color)
        .reserve(2)
        .push_powered(self.powered)
        .push_slice(&self.label)
        .pack()
    }
}

impl Pack for GetLightPower {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for SetLightPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_powered(self.powered)
        .push(self.duration)
        .pack()
    }
}

impl Pack for StateLightPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push_powered(self.powered)
        .pack()
    }
}

impl Pack for GetInfrared {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateInfrared {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.brightness)
        .pack()
    }
}

impl Pack for SetInfrared {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.brightness)
        .pack()
    }
}


// MultiZone Message
impl Pack for SetExtendedColorZones {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.duration)
        .push(self.apply as u8)
        .push(self.index)
        .push(self.colors_count)
        .push_all(&self.colors)
        .pack()
    }
}

impl Pack for GetExtendedColorZones {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateExtendedColorZones {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.duration)
        .push(self.apply as u8)
        .push(self.index)
        .push(self.colors_count)
        .push_all(&self.colors)
        .pack()
    }
}

impl Pack for SetColorZones {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.start_index)
        .push(self.end_index)
        .push(self.color)
        .push(self.duration)
        .push(self.apply as u8)
        .pack()
    }
}

impl Pack for GetColorZones {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.start_index)
        .push(self.end_index)
        .pack()
    }
}

impl Pack for StateZone {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.count)
        .push(self.index)
        .push(self.color)
        .pack()
    }
}

impl Pack for StateMultiZone {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
        .push(self.count)
        .push(self.index)
        .push_all(&self.colors)
        .pack()
    }
}


// Tile Messages
impl Pack for GetDeviceChain {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateDeviceChain {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.start_index)
            .push_all(&self.tile_devices)
            .push(self.total_count)
            .pack()
    }
}

impl Pack for SetUserPosition {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.tile_index)
            .reserve(2)
            .push(self.user_x)
            .push(self.user_y)
            .pack()
    }
}

impl Pack for GetTileState64 {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.tile_index)
            .push(self.length)
            .reserve(1)
            .push(self.x)
            .push(self.y)
            .push(self.width)
            .pack()
    }
}

impl Pack for StateTileState64 {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.tile_index)
            .reserve(1)
            .push(self.x)
            .push(self.y)
            .push(self.width)
            .push_all(&self.colors)
            .pack()
    }
}

impl Pack for SetTileState64 {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.tile_index)
            .push(self.length)
            .reserve(1)
            .push(self.x)
            .push(self.y)
            .push(self.width)
            .push_all(&self.colors)
            .pack()
    }
}

// Switch Messages
impl Pack for GetRelayPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.relay_index)
            .pack()
    }
}

impl Pack for SetRelayPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.relay_index)
            .push_powered(self.powered)
            .pack()
    }
}

impl Pack for StateRelayPower {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.relay_index)
            .push_powered(self.powered)
            .pack()
    }
}

// Firmware Effects
impl Pack for SetMultiZoneEffect {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.instanceid)
            .push(self.etype as u8)
            .reserve(2)
            .push(self.speed)
            .push(self.duration)
            .reserve(8)
            .push_all(&self.parameters)
            .pack()
    }
}

impl Pack for GetMultiZoneEffect {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateMultiZoneEffect {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .push(self.instanceid)
            .push(self.etype as u8)
            .reserve(2)
            .push(self.speed)
            .push(self.duration)
            .reserve(8)
            .push_all(&self.parameters)
            .pack()
    }
}

impl Pack for SetTileEffect {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .reserve(2)
            .push(self.instanceid)
            .push(self.etype as u8)
            .push(self.speed)
            .push(self.duration)
            .reserve(8)
            .push_all(&self.parameters)
            .push(self.palette_count)
            .push_all(&self.palette)
            .pack()
    }
}

impl Pack for GetTileEffect {
    fn pack(&self) -> Vec<u8> { Vec::new() }
}

impl Pack for StateTileEffect {
    fn pack(&self) -> Vec<u8> {
        PackHelper::new()
            .reserve(2)
            .push(self.instanceid)
            .push(self.etype as u8)
            .push(self.speed)
            .push(self.duration)
            .reserve(8)
            .push_all(&self.parameters)
            .push(self.palette_count)
            .push_all(&self.palette)
            .pack()
    }
}

// Tests

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
    
    let test = pack(
        &Header {
            tagged: true,
            source: 0,
            target: MacAddress::All,
            res_required: false,
            ack_required: false,
            sequence: 0,
        },
        &SetColor {
            color: HSBK::new(120.0, u16::MAX, u16::MAX, 3500),
            duration: 1024,
        },
    );

    let result = example.iter().zip(test.iter())
        .filter(|(a, b)| a != b)
        .enumerate()
        .collect::<Vec<_>>();

    if !result.is_empty() {
        panic!("Mismatched bytes: {:?}", result);
    }
}
