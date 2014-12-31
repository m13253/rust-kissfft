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