fn get_int() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i64>().unwrap()
}

fn get_float_vec() -> Vec<f64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let split = buffer.split_whitespace();
    let vec: Vec<&str> = split.collect();
    vec.iter().map(|x| x.trim().parse::<f64>().unwrap()).collect()
}

fn main() {
    let n: i64 = get_int(); 

    for _ in 0..n { 
        let v: Vec<f64> = get_float_vec();
        println!("{:.1}", v[0]*0.2+v[1]*0.3+v[2]*0.5); 
    }
}
