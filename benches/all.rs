use criterion::criterion_main;

mod fft;
mod field;
mod hash;
mod polynom;

criterion_main!(field::group, hash::group, fft::group, polynom::group);
