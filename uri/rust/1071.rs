fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}


fn main() {
    let (mut a,mut b,mut s): (i64,i64,i64) = (get_int(),get_int(),0); 

    if a > b { a ^= b; b ^= a; a ^= b; }

    if a % 2 != 0 { a+=1; }

    for n in (a+1..b).step_by(2) { s+=n; }
    
    println!("{}", s);
}
