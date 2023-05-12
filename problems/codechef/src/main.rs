fn main() {
    // let mut first_line=String::new();
    // std::io::stdin().read_line(&mut first_line);
    
    let num: Vec<&str> = "12 9".split(' ').collect();
    let f= num[0].parse::<u32>().unwrap();
    let g= num[1].parse::<u32>().unwrap();
    println!("{}, {:?}", f,g);
}
