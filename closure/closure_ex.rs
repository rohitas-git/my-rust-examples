fn main(){

    arr_desc_v1();

    arr_desc_v2();

    arr_desc_v0();
}

fn arr_desc_v0(){
    // arr.sort(); === arr.sort_by(|a, b| a.cmp(b));

    // Both statements give inverted order
    // arr.sort_by(|a, b| (&-*a).cmp(&-*b)); //? What?How?
    // arr.sort_by(|a, b| b.cmp(a));    

    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);
}

fn arr_desc_v1(){
    let mut arr=[4, 8, 1, 10, 0, 45, 12, 7];
    
    use std::cmp::Ordering;
    let desc= |a:&i32, b:&i32| -> Ordering{
        if a < b {Ordering::Greater}
        else if a > b {Ordering::Less}
        else {Ordering::Equal}
    }; 

    arr.sort_by(desc);
    println!("{:?}", arr);
}

fn arr_desc_v2(){
    let mut arr=[4, 8, 1, 10, 0, 45, 12, 7];

    use std::cmp::Ordering;
    arr.sort_by(
        |a, b|
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    );
    println!("{:?}", arr);
}