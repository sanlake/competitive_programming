fn get_float() -> f64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<f64>().unwrap()
}

fn main() {
    let mut n: i64 = 0;
    let mut aux: f64;

    for _ in 0..6 {
        aux = get_float();
        if aux > 0.0 { n+=1; }
    }

    println!("{} valores positivos", n);
}
