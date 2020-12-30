pub const LABEL_SZ: usize = 32;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min } else if val > max { max } else { val }
}

// transform skew ratio from [0, 1] to [-32768, 32767] per spec
pub fn conv_skew(skew_ratio: f32) -> u16 {
    (clamp(skew_ratio, 0.0, 1.0) * 65535.0 - 32768.0) as u16
}

// convert as much of label as possible to an array of valid utf-8 bytes
pub fn label_helper(label: &str) -> [u8; LABEL_SZ] {
    let mut bytes = [0u8; LABEL_SZ];
    let mut head = 0usize;
    let mut buf = [0u8; 4]; // essentially just a workspace for encode_utf8
    for ch in label.chars() {
        let ch_str = ch.encode_utf8(&mut buf).as_bytes();
        if head + ch_str.len() >= LABEL_SZ {
            break;
        }
        for i in 0..ch_str.len() {
            bytes[head] = ch_str[i];
            head += 1;
        }
    }

    bytes
}
