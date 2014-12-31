/*
Copyright (c) 2014-2015 StarBrilliant <m13253@hotmail.com>

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    * Neither the author nor the names of any contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

fn main() {
    let out_dir = match std::os::getenv("OUT_DIR") {
        Some(out_dir) => out_dir,
        None          => String::from_str(".")
    };
    let cc = match std::os::getenv("CC") {
        Some(cc) => cc,
        None     => String::from_str("gcc")
    };
    let ar = match std::os::getenv("AR") {
        Some(ar) => ar,
        None     => String::from_str("ar")
    };
    let compile_object = |filename: &str|
        std::io::Command::new(&cc)
            .args(&[String::from_str("-c"), String::from_str("-fPIC"), String::from_str("-o"), format!("{}/{}.o", out_dir, filename), format!("src/{}.c", filename)])
            .stdin(std::io::process::InheritFd(0)).stdout(std::io::process::InheritFd(1)).stderr(std::io::process::InheritFd(2))
            .status();
    let objects = ["kiss_fft", "kiss_fft_free"];
    for object in objects.iter() {
        assert!(compile_object(*object).unwrap().success());
    }
    let create_archive = |archive: &str, objects: &[&str]| {
        std::io::Command::new(&ar)
            .args(&["crs", format!("{}/lib{}.a", out_dir, archive).as_slice()])
            .args(objects.iter().map(|object: &&str| -> String format!("{}/{}.o", out_dir, *object)).collect::<Vec<String>>().as_slice())
            .stdin(std::io::process::InheritFd(0)).stdout(std::io::process::InheritFd(1)).stderr(std::io::process::InheritFd(2))
            .status()
    };
    assert!(create_archive("kissfft", &objects).unwrap().success());
    println!("cargo:rustc-flags=-L {} -l kissfft:static", out_dir);
}
