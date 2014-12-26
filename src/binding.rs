
#![allow(dead_code)]
#![allow(non_camel_case_types)] // Use the same spelling as the C header

extern crate libc;

#[static_assert]
static USE_SIMD: bool = !cfg!(USE_SIMD); // USE_SIMD is not supported in this binding

#[cfg(FIXED_POINT = "32")]
pub type kiss_fft_scalar = i32;
#[cfg(all(FIXED_POINT, not(FIXED_POINT = "32")))]
pub type kiss_fft_scalar = i16;
#[cfg(not(FIXED_POINT))]
pub type kiss_fft_scalar = f32;

#[repr(C)]
pub struct kiss_fft_cpx {
    r: kiss_fft_scalar,
    i: kiss_fft_scalar
}
impl Copy for kiss_fft_cpx {}

#[repr(C)]
pub struct kiss_fft_state;
impl Copy for kiss_fft_state {}
pub type kiss_fft_cfg = *mut kiss_fft_state;

#[link = "kissfft"]
extern {
    pub fn kiss_fft_alloc(nfft: libc::c_int, inverse_fft: libc::c_int, mem: *mut libc::c_void, lenmem: *mut libc::size_t) -> kiss_fft_cfg;
    pub fn kiss_fft(cfg: kiss_fft_cfg, fin: *const kiss_fft_cpx, fout: *mut kiss_fft_cpx);
    pub fn kiss_fft_stride(cfg: kiss_fft_cfg, fin: *const kiss_fft_cpx, fout: *mut kiss_fft_cpx, fin_stride: libc::c_int);
    pub fn kiss_fft_cleanup();
    pub fn kiss_fft_next_fast_size(n: libc::c_int) -> libc::c_int;
}

pub unsafe fn kiss_fft_free(cfg: kiss_fft_cfg) {
    libc::free(cfg as *mut libc::c_void)
}

pub unsafe fn kiss_fftr_next_fast_size_real(n: libc::c_int) -> libc::c_int {
    kiss_fft_next_fast_size(((n+(1i as libc::c_int))>>1u)<<1u)
}
