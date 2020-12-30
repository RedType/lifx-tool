use num_derive::FromPrimitive;
use super::util;

pub trait Pack {
    fn pack(&self) -> Vec<u8>;
}

#[derive(Clone, Copy)]
pub enum ApplicationRequest {
    NoApply = 0,
    Apply = 1,
    ApplyOnly = 2,
}

impl Default for ApplicationRequest {
    fn default() -> Self {
        ApplicationRequest::NoApply
    }
}

#[derive(Clone, Copy)]
pub enum MultiZoneEffectType {
    Off = 0,
    Move = 1,
}

impl Default for MultiZoneEffectType {
    fn default() -> Self {
        MultiZoneEffectType::Off
    }
}

#[derive(Clone, Copy)]
pub enum TileEffectType {
    Off = 0,
    Morph = 2,
    Flame = 3,
}

impl Default for TileEffectType {
    fn default() -> Self {
        TileEffectType::Off
    }
}

#[derive(Clone, Copy)]
pub enum MacAddress {
    All,
    Eui48([u8; 6]),
    //Eui64([u8; 8]), // unimplemented
}

impl Default for MacAddress {
    fn default() -> Self {
        MacAddress::All
    }
}

#[derive(Clone, Copy)]
pub struct HSBK {
    hue: f64,
    saturation: u16,
    brightness: u16,
    kelvin: u16,
}

impl HSBK {
    pub fn new(
        hue: f64,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
    ) -> Self {
        HSBK {
            hue: util::clamp(hue, 0.0, 360.0),
            saturation,
            brightness,
            kelvin: util::clamp(kelvin, 2500, 9000)
        }
    }

    pub fn hue(&self) -> f64 { self.hue }

    pub fn saturation(&self) -> u16 { self.saturation }

    pub fn brightness(&self) -> u16 { self.brightness }

    pub fn kelvin(&self) -> u16 { self.kelvin }
}

impl Default for HSBK {
    fn default() -> Self {
        HSBK::new(0.0, 0, 0, 0) // ok because of the clamp call in new
    }
}

#[derive(Clone, Copy, Default)]
pub struct Tile {
    pub accel_meas_x: i16,
    pub accel_meas_y: i16,
    pub accel_meas_z: i16,
    pub user_x: f32,
    pub user_y: f32,
    pub width: u8,
    pub height: u8,
    pub device_vendor: u32,
    pub device_product: u32,
    pub device_version: u32,
    pub firmware_build: u64,
    pub firmware_minor: u16,
    pub firmware_major: u16,
}

impl Tile {
    pub fn new(
        accel_meas_x: i16, accel_meas_y: i16, accel_meas_z: i16,
        user_x: f32, user_y: f32,
        width: u8, height: u8,
        device_vendor: u32, device_product: u32, device_version: u32,
        firmware_build: u64, firmware_minor: u16, firmware_major: u16,
    ) -> Tile {
        Tile {
            accel_meas_x, accel_meas_y, accel_meas_z,
            user_x, user_y,
            width, height,
            device_vendor, device_product, device_version,
            firmware_build, firmware_minor, firmware_major,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Header {
    pub tagged: bool,
    pub source: u32,
    pub target: MacAddress,
    pub res_required: bool,
    pub ack_required: bool,
    pub sequence: u8,
}

// Device Messages
#[derive(Clone, Copy, Default)]
pub struct GetService {}
#[derive(Clone, Copy, Default)]
pub struct StateService { pub service: u8, pub port: u32 }
#[derive(Clone, Copy, Default)]
pub struct GetHostInfo {}
#[derive(Clone, Copy, Default)]
pub struct StateHostInfo { pub signal: f32, pub tx: u32, pub rx: u32 }
#[derive(Clone, Copy, Default)]
pub struct GetHostFirmware {}
#[derive(Clone, Copy, Default)]
pub struct StateHostFirmware {
    pub build: u64,
    pub version_minor: u16,
    pub version_major: u16,
}
#[derive(Clone, Copy, Default)]
pub struct GetWifiInfo {}
#[derive(Clone, Copy, Default)]
pub struct StateWifiInfo { pub signal: f32, pub tx: u32, pub rx: u32 }
#[derive(Clone, Copy, Default)]
pub struct GetWifiFirmware {}
#[derive(Clone, Copy, Default)]
pub struct StateWifiFirmware {
    pub build: u64,
    pub version_minor: u16,
    pub version_major: u16,
}
#[derive(Clone, Copy, Default)]
pub struct GetPower {}
#[derive(Clone, Copy, Default)]
pub struct SetPower { pub powered: bool }
#[derive(Clone, Copy, Default)]
pub struct StatePower { pub powered: bool }
#[derive(Clone, Copy, Default)]
pub struct GetLabel {}
#[derive(Clone, Copy, Default)]
pub struct SetLabel { pub label: [u8; util::LABEL_SZ] }
#[derive(Clone, Copy, Default)]
pub struct StateLabel { pub label: [u8; util::LABEL_SZ] }
#[derive(Clone, Copy, Default)]
pub struct GetVersion {}
#[derive(Clone, Copy, Default)]
pub struct StateVersion { pub vendor: u32, pub product: u32, pub version: u32 }
#[derive(Clone, Copy, Default)]
pub struct GetInfo {}
#[derive(Clone, Copy, Default)]
pub struct StateInfo { pub time: u64, pub uptime: u64, pub downtime: u64 }
#[derive(Clone, Copy, Default)]
pub struct Acknowledgement {}
#[derive(Clone, Copy, Default)]
pub struct GetLocation {}
#[derive(Clone, Copy, Default)]
pub struct SetLocation {
    pub location: [u8; 16],
    pub label: [u8; util::LABEL_SZ],
    pub updated_at: i64,
}
#[derive(Clone, Copy, Default)]
pub struct StateLocation {
    pub location: [u8; 16],
    pub label: [u8; util::LABEL_SZ],
    pub updated_at: i64,
}
#[derive(Clone, Copy, Default)]
pub struct GetGroup {}
#[derive(Clone, Copy, Default)]
pub struct SetGroup {
    pub group: [u8; 16],
    pub label: [u8; util::LABEL_SZ],
    pub updated_at: i64,
}
#[derive(Clone, Copy, Default)]
pub struct StateGroup {
    pub group: [u8; 16],
    pub label: [u8; util::LABEL_SZ],
    pub updated_at: i64,
}
#[derive(Clone, Copy, Default)]
pub struct EchoRequest { pub payload: [u8; 8] }
#[derive(Clone, Copy, Default)]
pub struct EchoResponse { pub payload: [u8; 8] }

// Light Messages
#[derive(Clone, Copy, Default)]
pub struct Get {}
#[derive(Clone, Copy, Default)]
pub struct SetColor { pub color: HSBK, pub duration: u32 }
#[derive(Clone, Copy, Default)]
pub struct SetWaveform {
    pub transient: bool,
    pub color: HSBK,
    pub period: u32,
    pub cycles: f32,
    pub skew_ratio: f32,
    pub waveform: u8,
}
#[derive(Clone, Copy, Default)]
pub struct SetWaveformOptional {
    pub transient: bool,
    pub color: HSBK,
    pub period: u32,
    pub cycles: f32,
    pub skew_ratio: f32,
    pub waveform: u8,
    pub set_hue: bool,
    pub set_saturation: bool,
    pub set_brightness: bool,
    pub set_kelvin: bool,
}
#[derive(Clone, Copy, Default)]
pub struct State {
    pub color: HSBK,
    pub powered: bool,
    pub label: [u8; util::LABEL_SZ],
}
#[derive(Clone, Copy, Default)]
pub struct GetLightPower {}
#[derive(Clone, Copy, Default)]
pub struct SetLightPower { pub powered: bool, pub duration: u32 }
#[derive(Clone, Copy, Default)]
pub struct StateLightPower { pub powered: bool }
#[derive(Clone, Copy, Default)]
pub struct GetInfrared {}
#[derive(Clone, Copy, Default)]
pub struct StateInfrared { pub brightness: u16 }
#[derive(Clone, Copy, Default)]
pub struct SetInfrared { pub brightness: u16 }

// MultiZone Message
#[derive(Clone, Copy)]
pub struct SetExtendedColorZones {
    pub duration: u32,
    pub apply: ApplicationRequest,
    pub index: u16,
    pub colors_count: u8,
    pub colors: [HSBK; 82],
}
impl Default for SetExtendedColorZones {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            apply: Default::default(),
            index: Default::default(),
            colors_count: Default::default(),
            colors: [Default::default(); 82],
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct GetExtendedColorZones {}
#[derive(Clone, Copy)]
pub struct StateExtendedColorZones {
    pub duration: u32,
    pub apply: ApplicationRequest,
    pub index: u16,
    pub colors_count: u8,
    pub colors: [HSBK; 82],
}
impl Default for StateExtendedColorZones {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            apply: Default::default(),
            index: Default::default(),
            colors_count: Default::default(),
            colors: [Default::default(); 82],
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct SetColorZones {
    pub start_index: u8,
    pub end_index: u8,
    pub color: HSBK,
    pub duration: u32,
    pub apply: ApplicationRequest,
}
#[derive(Clone, Copy, Default)]
pub struct GetColorZones { pub start_index: u8, pub end_index: u8 }
#[derive(Clone, Copy, Default)]
pub struct StateZone { pub count: u8, pub index: u8, pub color: HSBK }
#[derive(Clone, Copy)]
pub struct StateMultiZone {
    pub count: u8,
    pub index: u8,
    pub colors: [HSBK; 8],
}
impl Default for StateMultiZone {
    fn default() -> Self {
        Self {
            count: Default::default(),
            index: Default::default(),
            colors: [Default::default(); 8],
        }
    }
}

// Tile Messages
#[derive(Clone, Copy, Default)]
pub struct GetDeviceChain {}
#[derive(Clone, Copy, Default)]
pub struct StateDeviceChain {
    pub start_index: u8,
    pub tile_devices: [Tile; 16],
    pub total_count: u8,
}
#[derive(Clone, Copy, Default)]
pub struct SetUserPosition {
    pub tile_index: u8,
    pub user_x: f32,
    pub user_y: f32,
}
#[derive(Clone, Copy, Default)]
pub struct GetTileState64 {
    pub tile_index: u8,
    pub length: u8,
    pub x: u8,
    pub y: u8,
    pub width: u8,
}
#[derive(Clone, Copy)]
pub struct StateTileState64 {
    pub tile_index: u8,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub colors: [HSBK; 64],
}
impl Default for StateTileState64 {
    fn default() -> Self {
        Self {
            tile_index: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            colors: [Default::default(); 64],
        }
    }
}
#[derive(Clone, Copy)]
pub struct SetTileState64 {
    pub tile_index: u8,
    pub length: u8,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub duration: u32,
    pub colors: [HSBK; 64],
}
impl Default for SetTileState64 {
    fn default() -> Self {
        Self {
            tile_index: Default::default(),
            length: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            duration: Default::default(),
            colors: [Default::default(); 64],
        }
    }
}

// Switch Messages
#[derive(Clone, Copy, Default)]
pub struct GetRelayPower { pub relay_index: u8 }
#[derive(Clone, Copy, Default)]
pub struct SetRelayPower { pub relay_index: u8, pub powered: bool }
#[derive(Clone, Copy, Default)]
pub struct StateRelayPower { pub relay_index: u8, pub powered: bool }

// Firmware Effects
#[derive(Clone, Copy, Default)]
pub struct SetMultiZoneEffect {
    pub instanceid: u32,
    pub etype: MultiZoneEffectType,
    pub speed: u32,
    pub duration: u64,
    pub parameters: [u32; 8],
}
#[derive(Clone, Copy, Default)]
pub struct GetMultiZoneEffect {}
#[derive(Clone, Copy, Default)]
pub struct StateMultiZoneEffect {
    pub instanceid: u32,
    pub etype: MultiZoneEffectType,
    pub speed: u32,
    pub duration: u64,
    pub parameters: [u32; 8],
}
#[derive(Clone, Copy, Default)]
pub struct SetTileEffect {
    pub instanceid: u32,
    pub etype: TileEffectType,
    pub speed: u32,
    pub duration: u64,
    pub parameters: [u32; 8],
    pub palette_count: u8,
    pub palette: [HSBK; 16],
}
#[derive(Clone, Copy, Default)]
pub struct GetTileEffect {}
#[derive(Clone, Copy, Default)]
pub struct StateTileEffect {
    pub instanceid: u32,
    pub etype: TileEffectType,
    pub speed: u32,
    pub duration: u64,
    pub parameters: [u32; 8],
    pub palette_count: u8,
    pub palette: [HSBK; 16],
}

pub trait Message: Pack {
    fn get_type(&self) -> MessageType;
}

macro_rules! message_type_and_conv {
    { $($variant:ident = $value:expr),+ $(,)? } => {
        #[derive(Clone, Copy, FromPrimitive)]
        pub enum MessageType {
            $($variant = $value),+
        }

        $(impl Message for $variant {
            fn get_type(&self) -> MessageType {
                MessageType::$variant
            }
        })+
    };
}

message_type_and_conv! {
    GetService = 2,
    StateService = 3,
    GetHostInfo = 12,
    StateHostInfo = 13,
    GetHostFirmware = 14,
    StateHostFirmware = 15,
    GetWifiInfo = 16,
    StateWifiInfo = 17,
    GetWifiFirmware = 18,
    StateWifiFirmware = 19,
    GetPower = 20,
    SetPower = 21,
    StatePower = 22,
    GetLabel = 23,
    SetLabel = 24,
    StateLabel = 25,
    GetVersion = 32,
    StateVersion = 33,
    GetInfo = 34,
    StateInfo = 35,
    Acknowledgement = 45,
    GetLocation = 48,
    SetLocation = 49,
    StateLocation = 50,
    GetGroup = 51,
    SetGroup = 52,
    StateGroup = 53,
    EchoRequest = 58,
    EchoResponse = 59,
    Get = 101,
    SetColor = 102,
    SetWaveform = 103,
    SetWaveformOptional = 119,
    State = 107,
    GetLightPower = 116,
    SetLightPower = 117,
    StateLightPower = 118,
    GetInfrared = 120,
    StateInfrared = 121,
    SetInfrared = 122,
    SetExtendedColorZones = 510,
    GetExtendedColorZones = 511,
    StateExtendedColorZones = 512,
    SetColorZones = 501,
    GetColorZones = 502,
    StateZone = 503,
    StateMultiZone = 506,
    GetDeviceChain = 701,
    StateDeviceChain = 702,
    SetUserPosition = 703,
    GetTileState64 = 707,
    StateTileState64 = 711,
    SetTileState64 = 715,
    GetRelayPower = 816,
    SetRelayPower = 817,
    StateRelayPower = 818,
    SetMultiZoneEffect = 508,
    GetMultiZoneEffect = 507,
    StateMultiZoneEffect = 509,
    SetTileEffect = 719,
    GetTileEffect = 718,
    StateTileEffect = 720,
}
