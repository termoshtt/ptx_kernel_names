//! Rust impl of LLVM Bitcode parsing example
//! based on https://github.com/sheredom/llvm_bc_parsing_example

extern crate llvm_sys;
#[macro_use]
extern crate structopt;

use std::path::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
