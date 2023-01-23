fn main(){

    /* 
    let a = 7;
    let a_box: Box<i32>;
    let mut a_ref: &i32 = &a;
    print!("{} {};", a, *a_ref);
    
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    print!(" {} {} {}", a, *a_ref, *a_box);
    */
    

    let a = 7;
    let mut a_box: Box<i32>;
    let a_ref: &i32 = &a;
    print!("{} {};", a, a_ref);
    a_box = Box::new(a + 2);
    print!(" {} {} {};", a, a_ref, a_box);
    a_box = Box::new(*a_ref);
    println!(" {} {} {}", a, a_ref, a_box);
}