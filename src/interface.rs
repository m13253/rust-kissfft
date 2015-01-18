/*
Copyright (c) 2014-2015 StarBrilliant <m13253@hotmail.com>

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    * Neither the author nor the names of any contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

extern crate libc;

use std;
use binding;

pub type Scalar = binding::kiss_fft_scalar;
pub type Complex = binding::kiss_fft_cpx;

pub struct KissFFT {
    cfg: binding::kiss_fft_cfg,
    nfft: usize,
    nfft_rsqrt: Scalar
}

impl KissFFT {
    pub fn new(nfft: usize, inverse_fft: bool) -> KissFFT {
        let cfg = unsafe {
            binding::kiss_fft_alloc(nfft as libc::c_int, inverse_fft as libc::c_int, std::ptr::null_mut(), std::ptr::null_mut())
        };
        assert!(cfg != std::ptr::null_mut());
        KissFFT {
            cfg: cfg,
            nfft: nfft,
            nfft_rsqrt: std::num::Float::rsqrt(nfft as Scalar)
        }
    }
    pub fn transform<'a>(&'a mut self, fin: &[Complex], fout: &mut [Complex]) -> &'a mut KissFFT {
        assert!(fin.len() >= self.nfft);
        assert!(fout.len() >= self.nfft);
        assert!(self.cfg != std::ptr::null_mut());
        unsafe {
            binding::kiss_fft(self.cfg, fin.as_ptr(), fout.as_mut_ptr())
        }
        self
    }
    pub fn transform_to_vec(&mut self, fin: &[Complex]) -> Vec<Complex> {
        assert!(fin.len() >= self.nfft);
        assert!(self.cfg != std::ptr::null_mut());
        let mut result = std::iter::repeat(Complex {r: 0., i: 0.}).take(self.nfft).collect::<Vec<Complex>>();
        self.transform(fin, result.as_mut_slice());
        result
    }
    pub fn transform_norm<'a>(&'a mut self, fin: &[Complex], fout: &mut [Complex]) -> &'a mut KissFFT {
        self.transform(fin, fout);
        for i in fout.iter_mut() {
            *i = *i * self.nfft_rsqrt
        }
        self
    }
    pub fn transform_norm_to_vec(&mut self, fin: &[Complex]) -> Vec<Complex> {
        let mut result = self.transform_to_vec(fin);
        for i in result.iter_mut() {
            *i = *i * self.nfft_rsqrt
        }
        result
    }
}

impl Drop for KissFFT {
    fn drop(&mut self) {
        unsafe {
            binding::kiss_fft_free(self.cfg);
        }
        self.cfg = std::ptr::null_mut();
    }
}
impl Complex {
    pub fn new(r: Scalar, i: Scalar) -> Complex {
        Complex { r: r, i: i }
    }
}
impl std::fmt::String for Complex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.pad(&*format!("{}{:+}i", self.r, self.i))
    }
}
impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex { r: self.r + other.r, i: self.i + other.i }
    }
}
impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex { r: self.r - other.r, i: self.i - other.i }
    }
}
impl std::ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex { r: -self.r, i: -self.i }
    }
}
impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            r: self.r*other.r - self.i*other.i,
            i: self.r*other.i + self.i*other.r
        }
    }
}
impl std::ops::Mul<Scalar> for Complex {
    type Output = Complex;
    fn mul(self, other: Scalar) -> Complex {
        Complex { r: self.r*other, i: self.i*other }
    }
}
impl std::ops::Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let denominator = other.r*other.r - other.i*other.i;
        Complex {
            r: (self.r*other.r + self.i*other.i)/denominator,
            i: (self.i*other.r - self.r*other.i)/denominator
        }
    }
}
impl std::ops::Div<Scalar> for Complex {
    type Output = Complex;
    fn div(self, other: Scalar) -> Complex {
        Complex { r: self.r/other, i: self.i/other }
    }
}
impl Complex {
    pub fn abs(self) -> Scalar {
        std::num::Float::hypot(self.r, self.i)
    }
    pub fn arg(self) -> Scalar {
        std::num::Float::atan2(self.i, self.r)
    }
}
