fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let n: i64 = get_int(); 
    for i in (2..10000).step_by(n as usize) { println!("{}", i); }
}
