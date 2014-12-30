extern crate libc;

pub mod binding;

pub type Scalar = binding::kiss_fft_scalar;
pub type Complex = binding::kiss_fft_cpx;

pub struct KissFFT {
    cfg: binding::kiss_fft_cfg,
    nfft: uint
}

impl KissFFT {
    pub fn new(nfft: uint, inverse_fft: bool) -> KissFFT {
        let cfg = unsafe {
            binding::kiss_fft_alloc(nfft as libc::c_int, inverse_fft as libc::c_int, std::ptr::null_mut(), std::ptr::null_mut())
        };
        assert!(cfg != std::ptr::null_mut());
        KissFFT {
            cfg: cfg,
            nfft: nfft
        }
    }
    pub fn transform(&mut self, fin: &[Complex], fout: &mut [Complex]) {
        assert!(fin.len() >= self.nfft);
        assert!(fout.len() >= self.nfft);
        assert!(self.cfg != std::ptr::null_mut());
        unsafe {
            binding::kiss_fft(self.cfg, fin.as_ptr(), fout.as_mut_ptr())
        }
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
