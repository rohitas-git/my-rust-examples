
use std::ops::{Add, Sub};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ComplexNumber<Number>  
where Number:Sized
{
    real: Number,
    imaginary: Number,
}

pub type Complex32 = ComplexNumber<f32>;
pub type Complex64 = ComplexNumber<f64>;

impl<T> ComplexNumber<T> {

    #[cfg(has_const_fn)]
    #[inline]
    pub const fn new(re: T, im: T) -> Self {
        ComplexNumber { real: re, imaginary: im }
    }

    #[cfg(not(has_const_fn))]
    #[inline]
    pub fn new(re: T, im: T) -> Self {
        ComplexNumber { real: re, imaginary: im }
    }
}


impl<T:Clone> ComplexNumber<T> {
    #[inline]
    pub fn get_imaginary(self) -> Number {
        self.imaginary
    }
    #[inline]
    pub fn get_real(self) -> Number{
        self.real
    }
    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.real.clone() * self.real.clone() + self.imaginary.clone() * self.imaginary.clone()
    }
    #[inline]
    pub fn set_real(self,real:T){
        self.real = real;
    }
    #[inline]
    pub fn set_imaginary(self, imaginary:T){
        self.imaginary = imaginary;
    }
    #[inline]
    pub fn scale(&self, t: T) -> Self {
        Self::new(self.re.clone() * t.clone(), self.im.clone() * t)
    }
    #[inline]
    pub fn unscale(&self, t: T) -> Self {
        Self::new(self.re.clone() / t.clone(), self.im.clone() / t)
    }
    #[inline]
    pub fn raise_power(&self, exp: u32) -> Self {
        // Pow::pow(self, exp)
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}

impl Sub for ComplexNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}




