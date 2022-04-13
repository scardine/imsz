use clap::Parser;
use imsz::imsz;

fn main() {
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about =
        "The imsz library gets image size from \
         files, this is a demo application.")]
    struct Args {
        files: Vec<String>,
    }

    let args = Args::parse();

    for fname in &args.files {
        match imsz(fname) {
            Ok(info) => println!("{}: {}, {} x {}", fname, info.format, info.width, info.height),
            Err(error) => eprintln!("{}: {}", fname, error)
        }
    }
}
