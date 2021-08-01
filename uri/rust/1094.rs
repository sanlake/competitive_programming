fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let n: i64 = get_int(); 
    let (mut c,mut r,mut s): (i64,i64,i64) = (0,0,0);

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed");
        let split = buffer.split_whitespace();
        let v: Vec<&str> = split.collect();
        let a: i64 = v[0].trim().parse::<i64>().unwrap();

        if v[1] == "C" { c+=a; }
        else if v[1] == "R" { r+=a; }
        else if v[1] == "S" { s+=a; }
    }

    let t: i64 = c+r+s;

    println!("Total: {} cobaias", t);
    println!("Total de coelhos: {}", c);
    println!("Total de ratos: {}", r);
    println!("Total de sapos: {}", s);
    println!("Percentual de coelhos: {:.2} %", 100.0 * c as f64 / t as f64);
    println!("Percentual de ratos: {:.2} %", 100.0 * r as f64 / t as f64);
    println!("Percentual de sapos: {:.2} %", 100.0 * s as f64 / t as f64);
}
