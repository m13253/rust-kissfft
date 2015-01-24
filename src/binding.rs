/*
Copyright (c) 2003-2010 Mark Borgerding
Copyright (c) 2014-2015 StarBrilliant <m13253@hotmail.com>

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    * Neither the author nor the names of any contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#![allow(non_camel_case_types)]

extern crate libc;

#[allow(dead_code)]
#[static_assert]
const USE_SIMD: bool = !cfg!(USE_SIMD); // USE_SIMD is not supported in this binding

#[cfg(FIXED_POINT = "32")]
pub type kiss_fft_scalar = i32;
#[cfg(all(FIXED_POINT, not(FIXED_POINT = "32")))]
pub type kiss_fft_scalar = i16;
#[cfg(not(FIXED_POINT))]
pub type kiss_fft_scalar = f32;

#[derive(Clone, Copy, Show)]
#[repr(C)]
pub struct kiss_fft_cpx {
    pub r: kiss_fft_scalar,
    pub i: kiss_fft_scalar
}

#[derive(Copy)]
#[repr(C)]
pub struct kiss_fft_state;
pub type kiss_fft_cfg = *mut kiss_fft_state;

#[link = "kissfft"]
extern {
    pub fn kiss_fft_alloc(nfft: libc::c_int, inverse_fft: libc::c_int, mem: *mut libc::c_void, lenmem: *mut libc::size_t) -> kiss_fft_cfg;
    pub fn kiss_fft(cfg: kiss_fft_cfg, fin: *const kiss_fft_cpx, fout: *mut kiss_fft_cpx);
    pub fn kiss_fft_stride(cfg: kiss_fft_cfg, fin: *const kiss_fft_cpx, fout: *mut kiss_fft_cpx, fin_stride: libc::c_int);
    pub fn kiss_fft_free(cfg: kiss_fft_cfg);
    pub fn kiss_fft_cleanup();
    pub fn kiss_fft_next_fast_size(n: libc::c_int) -> libc::c_int;
}

pub unsafe fn kiss_fftr_next_fast_size_real(n: libc::c_int) -> libc::c_int {
    kiss_fft_next_fast_size(((n+1)>>1)<<1)
}
