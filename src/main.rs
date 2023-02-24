use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let flag = args.next().unwrap_or_else(|| help());

    let conv: fn(_, _) -> _ = match flag.as_str() {
        "-e" => base94::encode,
        "-d" => base94::decode as _,
        _ => help(),
    };

    let src = fs::read(
        fs::canonicalize(args.next().unwrap_or_else(|| help()))
            .expect("could not canonicalize argument"),
    )
    .expect("could not read file");
    fs::write(
        args.next().unwrap_or_else(|| help()),
        &conv(&src, base94::BASE95),
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
