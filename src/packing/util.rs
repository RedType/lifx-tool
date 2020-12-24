use std::cmp;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min } else if val > max { max } else { val }
}

pub fn label_helper(label: &str) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    let size = cmp::min(label.len(), 32);
    for i in 0..size {
        buffer[i] = label.as_bytes()[i];
    }
    buffer
}
