//! Rust impl of LLVM Bitcode parsing example
//! based on https://github.com/sheredom/llvm_bc_parsing_example

extern crate llvm_sys;
#[macro_use]
extern crate structopt;
extern crate failure;

use failure::err_msg;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::*;
use std::os::raw::c_char;
use std::ptr::null_mut;
use structopt::StructOpt;

type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input file
    input: String,

    /// Output file
    output: String,
}

struct MemoryBuffer(LLVMMemoryBufferRef);

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMemoryBuffer(self.0) }
    }
}

impl MemoryBuffer {
    fn new(filename: &str) -> Result<Self> {
        let input = CString::new(filename)?;
        let mut membuf: LLVMMemoryBufferRef = null_mut();
        let mut msg: *mut c_char = null_mut();
        let result = unsafe {
            LLVMCreateMemoryBufferWithContentsOfFile(
                input.into_raw(),
                &mut membuf as *mut LLVMMemoryBufferRef,
                &mut msg as *mut *mut c_char,
            )
        };
        if result != 0 {
            let msg = unsafe { CString::from_raw(msg) };
            return Err(err_msg(format!("Canont read input: {:?}", msg)));
        }
        Ok(MemoryBuffer(membuf))
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let _membuf = MemoryBuffer::new(&opt.input);
}
