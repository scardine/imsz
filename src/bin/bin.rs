extern crate argparse;
extern crate imsz;
use argparse::{ArgumentParser, StoreTrue, Collect, Print};
use imsz::imsz;


fn main() {
    struct Options {
        verbose: bool,
        files: Vec<String>,
    }

    let mut options = Options { verbose: false, files: [].to_vec()};
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("The imsz library gets image size from \
                            files, this is a demo application.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.add_option(&["-V", "--version"],
            Print(env!["CARGO_PKG_VERSION"].to_string()), "Display version and exits");
        ap.refer(&mut options.files)
            .add_argument("files", Collect, "List of files to process").required();
        ap.parse_args_or_exit();
    }

    for fname in options.files.iter() {
        let info = imsz(fname);
        println!("{}: {:?}", fname, info);
    }
}
