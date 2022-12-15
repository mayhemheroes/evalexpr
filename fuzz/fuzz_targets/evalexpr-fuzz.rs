#![no_main]
use libfuzzer_sys::fuzz_target;
use evalexpr::*;
use std::str;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            eval(in_string);
        },
        Err(..)=>()
    }
});
