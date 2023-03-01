fn main(){
    let qr = need_for_traits::quartic_root_f64(100f64);
    println!("{} 's quartic root: {}", qr * qr * qr * qr, qr);

    print!("{} {}",
        need_for_traits::quartic_root_f64(100f64),
        need_for_traits::quartic_root_f32(100f32)
    );
}

 mod need_for_traits{

    pub mod without_traits{
        pub fn quartic_root_f64(x: f64) -> f64 { x.sqrt().sqrt() }
        pub fn quartic_root_f32(x: f32) -> f32 { x.sqrt().sqrt() }
    }
    
}