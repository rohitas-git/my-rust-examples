// How to ease the decoding of Option or Result enum values

pub fn divide(num:f64, denom:f64) -> Result<f64,String> {
    if denom==0.{
    Err(format!("Divide by zero"))
    }else{
        Ok(num/denom)
    }
}

pub fn check_r_isOk(r: &Result<f64,String>){
    println!("r: {:?}",r);
    assert_eq!(true,r.is_ok());
}
pub fn check_r_isErr(r: &Result<f64,String>){
    println!("r: {:?}",r);
    assert_eq!(true,r.is_err());
}
pub fn check_r_unwrap(r: &Result<f64,String>){
    println!("r.unwrap: {:?}",r.as_ref().unwrap());
}

#[cfg(test)]

mod test{

    use super::*;

    #[test]
    fn r1_isOk(){
        let r1:Result<f64,String> = divide(8., 2.);
        check_r_isOk(&r1);
    }
    
    #[test]
    fn r1_isErr(){
        let r1:Result<f64,String> = divide(8., 2.);
        check_r_isErr(&r1);
    }

    #[test]
    fn r2_isOk(){
        let r2:Result<f64,String> = divide(8., 0.);    
        check_r_isOk(&r2);
    }

    #[test]
    fn r2_isErr(){        
        let r2:Result<f64,String> = divide(8., 0.);
        check_r_isErr(&r2);
    }

    #[test]
    fn r1_unwrap(){        
        let r1:Result<f64,String> = divide(8., 2.);
        check_r_unwrap(&r1);
    }

    #[test]
    fn r2_unwrap(){
        let r2:Result<f64,String> = divide(8., 0.);
        check_r_unwrap(&r2);
    }
}