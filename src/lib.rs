use std::io::{self, prelude::*};

pub const BASE94: &[u8;94] = br##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

pub fn encode(
    src: impl Iterator<Item = u8>,
    dest: &mut impl Write,
    allowed_bytes: &[u8],
) -> io::Result<()> {
    assert!(allowed_bytes.len() > 1);
    assert!((0..allowed_bytes.len() - 1).all(|i| allowed_bytes[i] < allowed_bytes[i + 1]));
    let base = allowed_bytes.len() as u128;
    let r = max(base).1;
    let m = 1 << r;
    let mut acc = 0;
    let mut max = 1;

    for i in src {
        acc += max * u128::from(i);
        max <<= 8;
        if max >= m {
            encode_internal(dest, allowed_bytes, acc % m, m)?;
            acc >>= r;
            max >>= r;
        }
    }

    encode_internal(dest, allowed_bytes, acc, max)
}

pub fn decode(
    src: impl Iterator<Item = u8>,
    dest: &mut impl Write,
    allowed_bytes: &[u8],
) -> io::Result<()> {
    assert!(allowed_bytes.len() > 1);
    assert!((0..allowed_bytes.len() - 1).all(|i| allowed_bytes[i] < allowed_bytes[i + 1]));
    let base = allowed_bytes.len() as u128;
    let r = max(base).0;
    let m = base.pow(r);
    let mut acc = 0;
    let mut max = 1;
    let mut bits_used = 0;
    let mut bits = 0;

    for i in src {
        let a = max
            * match allowed_bytes.binary_search(&i) {
                Ok(x) => x as u128,
                _ => {
                    continue;
                }
            };
        acc += a;
        max *= base;
        if max >= m {
            (bits_used, bits) = decode_internal(dest, acc, m, bits_used, bits)?;
            acc = 0;
            max = 1;
        }
    }

    decode_internal(dest, acc, max, bits_used, bits)?;
    Ok(())
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
    dest: &mut impl Write,
    mut acc: u128,
    mut max: u128,
    prev: u32,
    prev_bits: u8,
) -> io::Result<(u32, u8)> {
    max <<= prev;
    acc <<= prev;
    acc |= prev_bits as u128;

    while max > 255 {
        dest.write_all(&[acc as u8])?;

        acc /= 256;
        max /= 256;
    }

    Ok((log2(max), acc as u8))
}

fn encode_internal(
    dest: &mut impl Write,
    allowed_bytes: &[u8],
    mut acc: u128,
    mut max: u128,
) -> io::Result<()> {
    let to_base = allowed_bytes.len() as u128;
    while max > 0 {
        dest.write_all(&[allowed_bytes[(acc % to_base) as usize]])?;
        acc /= to_base;
        max /= to_base;
    }
    Ok(())
}
