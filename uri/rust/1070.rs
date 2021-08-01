fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}


fn main() {
    let mut n: i64 = get_int(); 

    if n%2 == 0 { n+=1; }

    for i in (n..n+12).step_by(2) { println!("{}", i); }
}
