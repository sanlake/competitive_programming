fn get_float() -> f64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<f64>().unwrap()
}

fn main() {
    let sa: f64 = get_float();
    let pe: f64; 

    if sa <= 400.00 { pe=1.15; }
    else if sa <= 800.00 { pe=1.12; }
    else if sa <= 1200.00 { pe=1.10; }
    else if sa<=2000.00 { pe=1.07; }
    else { pe=1.04; }
	
    println!("Novo salario: {:.2}", sa*pe);
    println!("Reajuste ganho: {:.2}", sa*(pe-1.00));
    println!("Em percentual: {} %", (((pe-1.00)*100.0)+0.5) as i64 );
}
