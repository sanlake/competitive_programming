fn get_float() -> f64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<f64>().unwrap()
}

fn main() {
    let (mut sa, mut irf): (f64,f64) = ( get_float(), 0.0 );

    if sa > 4500.0 { irf+=(sa-4500.0)*0.28; sa=4500.0; }
    if sa > 3000.0 { irf+=(sa-3000.0)*0.18; sa=3000.0; }
    if sa > 2000.0 { irf+=(sa-2000.0)*0.08; }

    if irf > 0.0 { println!("R$ {:.2}", irf); }
    else { println!("Isento"); }
}
