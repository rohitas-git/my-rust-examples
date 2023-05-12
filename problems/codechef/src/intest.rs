use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut buff = String::new();
    stdin.read_line(&mut buff);
    let mut parts = buff.trim().split(' ');
    let n = parts.nth(0).unwrap().parse::<usize>().unwrap();
    let divisor = parts.nth(0).unwrap().parse::<u64>().unwrap();

    let mut counter = 0;
    for _ in 0..n {
        buff.clear();
        stdin.read_line(&mut buff);
        let x = buff.trim().parse::<u64>().unwrap();
        if (x % divisor) == 0 {
            counter += 1;
        }
    }
    println!("{}", counter);
}