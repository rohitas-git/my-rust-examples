struct Complex {
    re: f64,
    im: f64,
}
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { 
        write!(
            f,
            "{} {} {}i",
            self.re,
            if self.im >= 0. { '+' } else { '-' }, 
            self.im.abs()
        ) 
    }
}
fn main(){
    let c1 = Complex { re: -2.3, im: 0. }; 
    let c2 = Complex { re: -2.1, im: -5.2 }; 
    let c3 = Complex { re: -2.2, im: 5.2 }; 
    print!("{},\n{},\n{} \n", c1, c2, c3);
}