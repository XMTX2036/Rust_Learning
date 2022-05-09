use num::complex::Complex;
fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    // let result2 = Cadd(a, b);
    println!("{} + {}i", result.re, result.im);
    // println!("{} + {}i", result2.re, result2.im);
}