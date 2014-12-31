Rust-KissFFT
============

Rust binding of KissFFT library.

Usage
-----

```rust
let mut kiss_fft = kissfft::KissFFT::new(1024, false);

let fin = [kissfft::Complex { r: 0.0, i: 0.0 }, ..1024];
let mut fout = [kissfft::Complex { r: 0.0, i, 0.0 }, ..1024];
kiss_fft.transform(&fin, &mut fout);
```

You can also use `transform_as_vec` or `transform_norm` if convenient.

License
-------

This library is licensed under BSD license.

See the COPYING file for more information.
