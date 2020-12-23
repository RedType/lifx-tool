pub type Packet = Vec<u8>;

pub enum MACAddress {
    All,
    Eui48([u8; 6]),
    Eui64([u8; 8]), // unimplemented
}

pub struct HSBK {
    hue: u16,
    saturation: u16,
    brightness: u16,
    kelvin: u16,
}

impl HSBK {
    pub fn new(
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
    ) -> Result<Self, String> {
        if kelvin <= 9000 && kelvin >= 2500 {
            Ok(HSBK { hue, saturation, brightness, kelvin })
        } else {
            Err("HSBK kelvin value must be between 2500 and 9000 (inclusive)"
                .to_string())
        }
    }

    pub fn hue(&self) -> u16 { self.hue }
    pub fn saturation(&self) -> u16 { self.saturation }
    pub fn brightness(&self) -> u16 { self.brightness }
    pub fn kelvin(&self) -> u16 { self.kelvin }

    pub fn pack(&self) -> [u8; 8] {
        let h = self.hue.to_le_bytes();
        let s = self.saturation.to_le_bytes();
        let b = self.brightness.to_le_bytes();
        let k = self.kelvin.to_le_bytes();
        [ h[0], h[1], s[0], s[1], b[0], b[1], k[0], k[1] ]
    }
}
