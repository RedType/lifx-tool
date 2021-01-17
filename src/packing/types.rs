use std::{
    convert::TryInto,
    fmt,
    str,
};
use anyhow::{Error, Result};
use crate::{
    error::LifxError,
    util,
};

#[derive(Default)]
pub struct Header {
    pub tagged: bool,
    pub source: u32,
    pub target: u64,
    pub res_required: bool,
    pub ack_required: bool,
    pub sequence: u8,
}

impl Header {
    pub const SIZE: usize = 36;
}

pub enum Message {
    GetService,
    StateService {
        service: u8,
        port: u32,
    },
    Get,
    SetColor {
        color: HSBK,
        duration: u32,
    },
    State {
        color: HSBK,
        power: u16,
        label: LifxString,
    },
}

pub struct HSBK {
    pub hue: u16,
    pub saturation: u16,
    pub brightness: u16,
    pub kelvin: u16,
}

impl HSBK {
    pub const SIZE: usize = 8;
    pub const KELVIN_MIN: u16 = 2500;
    pub const KELVIN_MAX: u16 = 9000;
    pub const HUE_MAX: f64 = 360.0;

    pub fn new(h: f64, s: u16, b: u16, k: u16) -> Self {
        HSBK {
            hue: (h / Self::HUE_MAX * u16::MAX as f64) as u16,
            saturation: s,
            brightness: b,
            kelvin: util::clamp(k, Self::KELVIN_MIN, Self::KELVIN_MAX),
        }
    }

    /// Creates an HSBK from the given H, S, V, and K values
    ///
    /// # Arguments
    ///
    /// * `h` - hue in degrees within [0.0, 360.0]
    /// * `s` - saturation within [0.0, 1.0]
    /// * `v` - value (lifx calls this brightness) within [0.0, 1.0]
    /// * `k` - light temperature in kelvins within [2500, 9000]
    pub fn from_hsvk(h: f64, s: f64, v: f64, k: u16) -> Self {
        assert_eq!(util::clamp(h, 0.0, Self::HUE_MAX), h);
        assert_eq!(util::clamp(s, 0.0, 1.0), s);
        assert_eq!(util::clamp(v, 0.0, 1.0), v);
        assert_eq!(util::clamp(k, Self::KELVIN_MIN, Self::KELVIN_MAX), k);

        HSBK {
            hue: (h / Self::HUE_MAX * u16::MAX as f64) as u16,
            saturation: (s * u16::MAX as f64) as u16,
            brightness: (v * u16::MAX as f64) as u16,
            kelvin: util::clamp(k, Self::KELVIN_MIN, Self::KELVIN_MAX),
        }
    }

    /// Creates an HSBK from the given R, G, B, and K values
    ///
    /// # Arguments
    ///
    /// * `r` - red color within [0.0, 1.0]
    /// * `g` - green color within [0.0, 1.0]
    /// * `b` - blue color within [0.0, 1.0]
    /// * `k` - light temperature in kelvins within [2500, 9000]
    pub fn from_rgbk(r: f64, g: f64, b: f64, k: u16) -> Self {
        assert_eq!(util::clamp(r, 0.0, 1.0), r);
        assert_eq!(util::clamp(g, 0.0, 1.0), g);
        assert_eq!(util::clamp(b, 0.0, 1.0), b);

        let c_max = util::max3(r, g, b);
        let c_min = util::min3(r, g, b);
        let delta = c_max - c_min;

        let v = c_max;

        let s = if c_max == 0.0 { 0.0 } else { delta / c_max };

        let mut h = 60.0 * if delta == 0.0 {
            0.0
        } else if c_max == r {
            (g - b) / delta
        } else if c_max == g {
            2.0 + (b - r) / delta
        } else { // c_max == b
            4.0 + (r - g) / delta
        };

        if h < 0.0 {
            h += Self::HUE_MAX;
        }

        Self::from_hsvk(h, s, v, k)
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut pack = Vec::new();
        pack.extend_from_slice(&self.hue.to_le_bytes());
        pack.extend_from_slice(&self.saturation.to_le_bytes());
        pack.extend_from_slice(&self.brightness.to_le_bytes());
        pack.extend_from_slice(&self.kelvin.to_le_bytes());
        pack
    }

    pub fn unpack(pack: &[u8]) -> Result<Self> {
        if pack.len() != Self::SIZE {
            return Err(Error::new(LifxError::WrongSize {
                found: pack.len(),
                expected: Self::SIZE,
            }));
        }

        Ok(Self {
            hue: u16::from_le_bytes(pack[0..=1].try_into()?),
            saturation: u16::from_le_bytes(pack[2..=3].try_into()?),
            brightness: u16::from_le_bytes(pack[4..=5].try_into()?),
            kelvin: u16::from_le_bytes(pack[6..=7].try_into()?),
        })
    }
}

pub struct LifxString(pub [u8; Self::SIZE]);

impl LifxString {
    pub const SIZE: usize = 32;

    // ensure that the result is a valid utf-8 string
    // (does not need to be null terminated)
    pub fn from_buf(b: &[u8]) -> Result<Self> {
        let s = str::from_utf8(b)?;
        let mut iter = s.chars();
        let mut tail = 0usize;
        let mut buf = [0u8; Self::SIZE];
        while let Some(c) = iter.next() {
            if tail + c.len_utf8() > Self::SIZE { break; }
            c.encode_utf8(&mut buf[tail..]);
            tail += c.len_utf8();
        }

        Ok(LifxString(buf))
    }
}

impl fmt::Display for LifxString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match str::from_utf8(&self.0) {
            Ok(s) => {
                write!(f, "{}", s)?;
                Ok(())
            },
            Err(_) => Err(fmt::Error),
        }
    }
}
