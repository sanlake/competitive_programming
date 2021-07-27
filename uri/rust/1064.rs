fn get_float() -> f64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<f64>().unwrap()
}


fn main() {
    let (mut s, mut q): (f64,i32) = (0.0,0);
    
    for _ in 0..6 { 
        let aux = get_float(); 
        if aux > 0.0 { s += aux; q += 1; }
    }

    println!("{} valores positivos\n{:.1}", q, s / q as f64);
}
