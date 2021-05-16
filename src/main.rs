extern crate nix;

use clap::{App, Arg};
use std::fs::{File, rename};
use std::io;
use std::os::unix::io::AsRawFd;

use nix::fcntl::{splice, SpliceFFlags};

const BUF_SIZE: usize = 16384;
const WRAP_AFTER: usize = 4 * BUF_SIZE;

fn main() {
    let stdin = io::stdin();
    let _handle = stdin.lock();
    let mut file_index = 0;

    let mut output = File::create("current.txt").expect(&format!("Could not create file"));
    let mut written_bytes = 0;

    loop {
        let res = splice(
            stdin.as_raw_fd(),
            None,
            output.as_raw_fd(),
            None,
            BUF_SIZE,
            SpliceFFlags::empty(),
        )
        .unwrap();

        if res == 0 {
            // We read 0 bytes from the input,
            // which means we're done copying.
            break;
        }

        written_bytes += res;
        if written_bytes >= WRAP_AFTER {
            written_bytes = 0;
            println!("Rotating file!");
            // "close" current file
            drop(output);
            // rename it to file_index
            rename("current.txt", file_index.to_string() + ".txt").unwrap();
            // reopen current
            output = File::create("current.txt").expect(&format!("Could not create new current.txt"));
            // increment file_index
            file_index = (file_index + 1) % 5;
        }
    }
}

fn _setup_arguments() -> clap::ArgMatches<'static> {
    let matches = App::new("rlog")
        .version("0.1.0")
        .author("Remko van Wagensveld")
        .about("Log stdin to rotating logfiles")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help("Set the path to log to.")
                .takes_value(true),
        )
        .get_matches();

    matches
}
