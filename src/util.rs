pub fn clamp<T: PartialOrd>(x: T, lo: T, hi: T) -> T {
    if x < lo { lo } else if x > hi { hi } else { x }
}

pub fn max3<T: PartialOrd>(a: T, b: T, c: T) -> T {
    let max_ab = if a > b { a } else { b };
    if max_ab > c { max_ab } else { c }
}

pub fn min3<T: PartialOrd>(a: T, b: T, c: T) -> T {
    let min_ab = if a < b { a } else { b };
    if min_ab < c { min_ab } else { c }
}
