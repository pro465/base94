pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    for c in src.chunks(9) {
        conv(c, &mut res, 256, 94, 1, 11);
    }
    res.iter_mut().for_each(|i| *i += 33);
    res
}

pub fn decode(src: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let mut i = 0;

    while i < src.len() {
        let mut c = [0; 11];
        let mut len = 0;
        while i < src.len() && len < 11 {
            if (33..127).contains(&src[i]) {
                c[len] = src[i] - 33;
                len += 1;
            }
            i += 1;
        }
        conv(&c[..len], &mut res, 94, 256, 255, 9);
    }
    res
}

fn conv(
    batch: &[u8],
    res: &mut Vec<u8>,
    from_base: u128,
    to_base: u128,
    min: u128,
    max_len: usize,
) {
    let mut i = 0;
    let mut max = 1;
    for &d in batch.iter().rev() {
        i *= from_base;
        i += d as u128;
        max *= from_base;
    }
    max -= 1;
    for _ in 0..max_len {
        if max < min {
            break;
        }
        res.push((i % to_base) as u8);
        i /= to_base;
        max /= to_base;
    }
}
