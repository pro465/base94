use std::fs::{self, File};
use std::io::prelude::*;
fn main() {
    let mut args = std::env::args().skip(1);
    let flag = args.next().unwrap_or_else(|| help());

    let conv: fn(_, _, _) -> _ = match flag.as_str() {
        "-e" => base94::encode,
        "-d" => base94::decode as _,
        _ => help(),
    };

    let src = File::open(
        fs::canonicalize(args.next().unwrap_or_else(|| help()))
            .expect("could not canonicalize argument"),
    )
    .expect("could not open file")
    .bytes()
    .map(|i| i.expect("could not read file"));

    conv(
        src,
        &mut File::create(args.next().unwrap_or_else(|| help()))
            .expect("could not create destination file"),
        base94::BASE94,
    )
    .expect("could not write to file");
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
