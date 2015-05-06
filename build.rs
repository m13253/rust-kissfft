/*
Copyright (c) 2014-2015 StarBrilliant <m13253@hotmail.com>

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    * Neither the author nor the names of any contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#![feature(collections)]
extern crate collections;

use collections::string::String;
use std::process::Stdio;

fn main() {
/* Note:
   The GCC crate ( https://crates.io/crates/gcc ) is currently somewhat buggy,
   I will switch to it only when it is stable enough. */
    let out_dir = std::env::var("OUT_DIR").unwrap_or(String::from_str("."));
    let cc = std::env::var("CC").unwrap_or(String::from_str("gcc"));
    let ar = std::env::var("AR").unwrap_or(String::from_str("ar"));
    let compile_object = |filename: &str|
        std::process::Command::new(&cc)
            .args(&["-c", "-fPIC", "-O3", "-Wall", "-o"])
            .args(&[format!("{}/{}.o", out_dir, filename), format!("src/{}.c", filename)])
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
            .status();
    let objects = ["kiss_fft", "kiss_fft_free"];
    for object in objects.iter() {
        assert!(compile_object(*object).unwrap().success());
    }
    let create_archive = |archive: &str, objects: &[&str]| {
        std::process::Command::new(&ar)
            .args(&["crs", &*format!("{}/lib{}.a", out_dir, archive)])
            .args(&*objects.iter().map(|object: &&str| format!("{}/{}.o", out_dir, *object)).collect::<Vec<String>>())
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
            .status()
    };
    assert!(create_archive("kissfft", &objects).unwrap().success());
    println!("cargo:rustc-flags=-L {}", out_dir);
}
