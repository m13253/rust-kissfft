extern crate libc;

use std;
use binding;

pub type Scalar = binding::kiss_fft_scalar;
pub type Complex = binding::kiss_fft_cpx;

pub struct KissFFT {
    cfg: binding::kiss_fft_cfg,
    nfft: uint,
    nfft_rsqrt: Scalar
}

impl KissFFT {
    pub fn new(nfft: uint, inverse_fft: bool) -> KissFFT {
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
        let mut result = Vec::from_elem(self.nfft, Complex {r: 0., i: 0.});
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
impl Add<Complex, Complex> for Complex {
    fn add(self, other: Complex) -> Complex {
        Complex { r: self.r + other.r, i: self.i + other.i }
    }
}
impl Sub<Complex, Complex> for Complex {
    fn sub(self, other: Complex) -> Complex {
        Complex { r: self.r - other.r, i: self.i - other.i }
    }
}
impl Neg<Complex> for Complex {
    fn neg(self) -> Complex {
        Complex { r: -self.r, i: -self.i }
    }
}
impl Mul<Complex, Complex> for Complex {
    fn mul(self, other: Complex) -> Complex {
        Complex {
            r: self.r*other.r - self.i*other.i,
            i: self.r*other.i + self.i*other.r
        }
    }
}
impl Mul<Scalar, Complex> for Complex {
    fn mul(self, other: Scalar) -> Complex {
        Complex { r: self.r*other, i: self.i*other }
    }
}
impl Div<Complex, Complex> for Complex {
    fn div(self, other: Complex) -> Complex {
        let denominator = other.r*other.r - other.i*other.i;
        Complex {
            r: (self.r*other.r + self.i*other.i)/denominator,
            i: (self.i*other.r - self.r*other.i)/denominator
        }
    }
}
impl Div<Scalar, Complex> for Complex {
    fn div(self, other: Scalar) -> Complex {
        Complex { r: self.r/other, i: self.i/other }
    }
}
impl Complex {
    pub fn abs(self) -> Scalar {
        std::num::FloatMath::hypot(self.r, self.i)
    }
    pub fn arg(self) -> Scalar {
        std::num::FloatMath::atan2(self.i, self.r)
    }
}