#![cfg(test)]

use std;
use interface;

#[test]
fn test_a() {
    let nfft_sqrt = 32u;
    let nfft = nfft_sqrt * nfft_sqrt;
    let mut kiss_fft = interface::KissFFT::new(nfft, false);
    let mut kiss_ifft = interface::KissFFT::new(nfft, true);
    let fin = Vec::from_fn(nfft, |_| interface::Complex {r: std::rand::random::<interface::Scalar>(), i: std::rand::random::<interface::Scalar>()} );
    let fout_fft = kiss_fft.transform_to_vec(fin.as_slice()) .iter().map(|&x| interface::Complex {r: x.r/32., i: x.i/32.}).collect::<Vec<interface::Complex>>();
    let fout_ifft = kiss_ifft.transform_to_vec(fout_fft.as_slice()) .iter().map(|&x| interface::Complex {r: x.r/32., i: x.i/32.}).collect::<Vec<interface::Complex>>();
    println!("\nIN   = [\n{}\n]", fin.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    println!("\nFFT  = [\n{}\n]", fout_fft.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    println!("\nIFFT = [\n{}\n]", fout_ifft.iter().map(|x: &interface::Complex| format!("    {}", x)).collect::<Vec<String>>().connect(",\n"));
    for (i, j) in fin.iter().zip(fout_ifft.iter()) {
        assert!((std::num::FloatMath::abs_sub(i.r, j.r) >= 0.01) || (std::num::FloatMath::abs_sub(i.i, j.i) >= 0.01));
    }
}