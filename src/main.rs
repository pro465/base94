use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let flag = args.next().unwrap_or_else(|| help());

    /*    let (mut m, mut r) = (f64::INFINITY, (0, 0));
        for i in 0..19 {
            let a: u128 = 94_u128.pow(i + 1);
            let mut j = 0;
            while a >= 1 << (j + 1) {
                j += 1;
            }
            let score = (8 * i + 8) as f64 / j as f64;
            if score < m {
                m = score;
                r = (i + 1, j);
            }
        }

        dbg!(r, m);
    */

    let conv: fn(_) -> _ = match flag.as_str() {
        "-e" => base94::encode,
        "-d" => base94::decode as _,
        _ => help(),
    };

    let src = fs::read(
        fs::canonicalize(args.next().unwrap_or_else(|| help()))
            .expect("could not canonicalize argument"),
    )
    .expect("could not read file");
    fs::write(args.next().unwrap_or_else(|| help()), &conv(&src)).expect("could not write to file");
}

fn help() -> ! {
    println!(
        r"usage: {} -FLAG <src_filename> <dest_filename>
FLAG:
   -e 
   encode src file and write the encoded data to dest file
   -d
   decode src file and write the decoded data to dest file",
        std::env::current_exe()
            .unwrap_or_else(|_| "base94".into())
            .display()
    );
    std::process::exit(-1);
}
/*
fn encode(src: Vec<u8>, mut dest: impl Write) {
    for c in src.chunks(9) {
        let mut res = conv(c, 256, 94, 0);
        res.truncate(11);
        res.iter_mut().for_each(|i| *i += 33);
        dest.write_all(&res).unwrap();
    }
}

fn decode(mut src: Vec<u8>, mut dest: impl Write) {
    src.retain(|i| (33..127).contains(i));

    for c in src.chunks_mut(11) {
        c.iter_mut().for_each(|i| *i -= 33);
        let mut res = conv(c, 94, 256, 254);
        res.truncate(9);
        dest.write_all(&res).unwrap();
    }
}

fn conv(batch: &[u8], from_base: u128, to_base: u128, min: u128) -> Vec<u8> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut max = 1;
    for &d in batch.iter().rev() {
        i *= from_base;
        i += d as u128;
        max *= from_base;
    }
    max -= 1;
    while max > min {
        res.push((i % to_base) as u8);
        i /= to_base;
        max /= to_base;
    }
    res
}*/
