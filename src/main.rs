//! Rust impl of LLVM Bitcode parsing example
//! based on https://github.com/sheredom/llvm_bc_parsing_example

extern crate llvm_sys;
#[macro_use]
extern crate structopt;
extern crate failure;

use llvm_sys::bit_reader::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::LLVMLinkage;

use failure::err_msg;
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

#[derive(Debug)]
struct Module(LLVMModuleRef);

#[derive(Debug)]
struct Function(LLVMValueRef);

impl Module {
    fn parse_bitcode(buf: &MemoryBuffer) -> Result<Self> {
        let mut md: LLVMModuleRef = null_mut();
        let res = unsafe { LLVMParseBitcode2(buf.0, &mut md as *mut _) };
        if res != 0 {
            return Err(err_msg("Cannot parse LLVM Bitcode"));
        }
        Ok(Module(md))
    }

    fn functions(&self) -> Vec<Function> {
        let mut funcs = Vec::new();
        let mut f = unsafe { LLVMGetFirstFunction(self.0) };
        while f != null_mut() {
            funcs.push(Function(f));
            f = unsafe { LLVMGetNextFunction(f) };
        }
        funcs
    }
}

impl Function {
    fn name(&self) -> String {
        let name = unsafe { CString::from_raw(LLVMGetValueName(self.0) as *mut _) };
        name.into_string().expect("Fail to parse function name")
    }

    fn linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }

    fn type_of(&self) -> LLVMTypeRef {
        unsafe { LLVMTypeOf(self.0) }
    }

    fn call_conv(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let membuf = MemoryBuffer::new(&opt.input)?;
    let md = Module::parse_bitcode(&membuf)?;

    for func in md.functions() {
        println!("Func       = {}", func.name());
        println!("Linkage    = {:?}", func.linkage());
        println!("Type       = {:?}", func.type_of());
        println!("Call conv  = {:?}", func.call_conv());
    }
    Ok(())
}
