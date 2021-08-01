fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let n: i64 = get_int(); 
    for i in 1..=10 { println!("{} x {} = {}", i,n,i*n); }
}
