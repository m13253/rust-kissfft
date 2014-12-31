/*
Copyright (c) 2014-2015 StarBrilliant <m13253@hotmail.com>

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    * Neither the author nor the names of any contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#![cfg(test)]

use std;
use interface;

#[test]
fn test_random_fft() {
    let nfft = 1024;
    let mut kiss_fft = interface::KissFFT::new(nfft, false);
    let mut kiss_ifft = interface::KissFFT::new(nfft, true);
    let fin = Vec::from_fn(nfft, |_| interface::Complex {r: std::rand::random::<interface::Scalar>()*2.-1., i: std::rand::random::<interface::Scalar>()*2.-1.} );
    let fout_fft = kiss_fft.transform_norm_to_vec(fin.as_slice());
    let fout_ifft = kiss_ifft.transform_norm_to_vec(fout_fft.as_slice());
    println!("\nIN   = [\n{}\n]", fin.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    println!("\nFFT  = [\n{}\n]", fout_fft.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    println!("\nIFFT = [\n{}\n]", fout_ifft.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    for (i, j) in fin.iter().zip(fout_ifft.iter()) {
        assert!((*i - *j).abs() < 0.001);
    }
}
