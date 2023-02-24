pub const BASE94: &[u8;94] = br##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;
pub const BASE95: &[u8;95] = br##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

pub fn encode(src: &[u8], allowed_bytes: &[u8]) -> Vec<u8> {
    assert!((0..allowed_bytes.len() - 1).all(|i| allowed_bytes[i] < allowed_bytes[i + 1]));
    let mut res = Vec::new();
    let base = allowed_bytes.len() as u128;
    let r = max(base).1;
    let m = 1 << r;
    let mut i = 0;
    let mut acc = 0;
    let mut max = 1;

    while i < src.len() {
        acc += max * u128::from(src[i]);
        max <<= 8;
        if max >= m {
            encode_internal(&mut res, allowed_bytes, acc % m, m);
            acc >>= r;
            max >>= r;
        }
        i += 1;
    }

    encode_internal(&mut res, allowed_bytes, acc, max);

    res
}

pub fn decode(src: &[u8], allowed_bytes: &[u8]) -> Vec<u8> {
    assert!((0..allowed_bytes.len() - 1).all(|i| allowed_bytes[i] < allowed_bytes[i + 1]));
    let mut res = Vec::new();
    let base = allowed_bytes.len() as u128;
    let r = max(base).0;
    let m = base.pow(r);
    let mut i = 0;
    let mut acc = 0;
    let mut max = 1;
    let mut bits_used = 0;
    let mut bits = 0;

    while i < src.len() {
        let a = max
            * match allowed_bytes.binary_search(&src[i]) {
                Ok(x) => x as u128,
                _ => {
                    i += 1;
                    continue;
                }
            };
        acc += a;
        max *= base;
        if max >= m {
            (bits_used, bits) = decode_internal(&mut res, acc, m, bits_used, bits);
            acc = 0;
            max = 1;
        }
        i += 1;
    }

    decode_internal(&mut res, acc, max, bits_used, bits);

    res
}

fn log2(i: u128) -> u32 {
    let mut j = 0;
    while i >= 1 << (j + 1) {
        j += 1;
    }
    j
}

fn max(base: u128) -> (u32, u32) {
    let (mut m, mut r) = (f64::INFINITY, (0, 0));
    let mut a: u128 = 1;
    let mut i = 0;

    while let Some(new) = a.checked_mul(base) {
        a = new;
        i += 1;
        let j = log2(a);
        let score = (8 * i) as f64 / j as f64;
        if score < m {
            m = score;
            r = (i, j);
        }
    }

    r
}

fn decode_internal(
    res: &mut Vec<u8>,
    mut acc: u128,
    mut max: u128,
    prev: u32,
    prev_bits: u8,
) -> (u32, u8) {
    max <<= prev;
    acc <<= prev;
    acc |= prev_bits as u128;

    while max > 255 {
        res.push(acc as u8);

        acc /= 256;
        max /= 256;
    }

    (log2(max), acc as u8)
}

fn encode_internal(res: &mut Vec<u8>, allowed_bytes: &[u8], mut acc: u128, mut max: u128) {
    let to_base = allowed_bytes.len() as u128;
    while max > 0 {
        res.push(allowed_bytes[(acc % to_base) as usize]);
        acc /= to_base;
        max /= to_base;
    }
}
