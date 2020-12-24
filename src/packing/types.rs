use super::util;

pub type Packet = Vec<u8>;

pub enum MACAddress {
    All,
    Eui48([u8; 6]),
    Eui64([u8; 8]), // unimplemented
}

pub enum ApplicationRequest {
    NoApply = 0,
    Apply = 1,
    ApplyOnly = 2,
}

pub enum MultiZoneEffectType {
    Off = 0,
    Move = 1,
}

pub enum TileEffectType {
    Off = 0,
    Morph = 2,
    Flame = 3,
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

    pub fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = Vec::new();
        // hue value is scaled from 0-360 to 0-65535
        let hue_conv = (self.hue / 360.0 * 65535.0) as u16;
        packed.extend_from_slice(&hue_conv.to_le_bytes());
        packed.extend_from_slice(&self.saturation.to_le_bytes());
        packed.extend_from_slice(&self.brightness.to_le_bytes());
        packed.extend_from_slice(&self.kelvin.to_le_bytes());
        packed
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

    pub fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = Vec::new();
        packed.extend_from_slice(&self.accel_meas_x.to_le_bytes());
        packed.extend_from_slice(&self.accel_meas_y.to_le_bytes());
        packed.extend_from_slice(&self.accel_meas_z.to_le_bytes());
        packed.extend_from_slice(&[0u8; 2]); // reserved
        packed.extend_from_slice(&self.user_x.to_le_bytes());
        packed.extend_from_slice(&self.user_y.to_le_bytes());
        packed.push(self.width);
        packed.push(self.height);
        packed.push(0u8); // reserved
        packed.extend_from_slice(&self.device_vendor.to_le_bytes());
        packed.extend_from_slice(&self.device_product.to_le_bytes());
        packed.extend_from_slice(&self.device_version.to_le_bytes());
        packed.extend_from_slice(&self.firmware_build.to_le_bytes());
        packed.extend_from_slice(&[0u8; 8]); // reserved
        packed.extend_from_slice(&self.firmware_minor.to_le_bytes());
        packed.extend_from_slice(&self.firmware_major.to_le_bytes());
        packed.extend_from_slice(&[0u8; 4]); // reserved
        packed
    }
}
